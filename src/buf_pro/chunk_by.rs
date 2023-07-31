fn search_in<T: PartialEq>(input: &[T], iden: &[T]) -> Option<usize> {
    input
        .windows(iden.len())
        .enumerate()
        .find_map(|(i, w)| if w == iden { Some(i) } else { None })
}
fn filter_in<T: PartialEq>(input: &[T], iden: &[T]) -> Vec<usize> {
    input
        .windows(iden.len())
        .enumerate()
        .filter_map(|(i, w)| if w == iden { Some(i) } else { None })
        .collect()
}
pub fn chunkBy_once<'a, T: PartialEq>(input: &'a [T], iden: &[T]) -> Vec<&'a [T]> {
    let mut v = Vec::new();

    {
        let mut o = filter_in(input, iden);
        o.push(input.len());
        o
    }
    .windows(2)
    .for_each(|i| v.push(&input[i[0]..i[1]]));

    v
}
pub struct ChunkerBy<T> {
    iden: Vec<T>,
    buffer: Vec<T>,
    lastStart: Option<usize>,
}

impl<T: PartialEq + Clone> ChunkerBy<T> {
    pub fn new(iden: &[T]) -> Self {
        Self {
            iden: iden.to_vec(),
            buffer: Vec::new(),
            lastStart: None,
        }
    }
    fn extract_cache(&mut self) {
        if let Some(lastStart) = self.lastStart.take() {
            self.buffer.drain(..lastStart);
        }
    }
    pub fn flush(&mut self) -> &[T] {
        self.extract_cache();
        &self.buffer
    }
    pub fn chunk<'a>(&'a mut self, data_input: &[T]) -> Vec<&'a [T]> {
        if data_input.is_empty() {
            return Vec::new();
        }
        self.extract_cache();

        self.buffer.extend_from_slice(data_input);

        let mut v = Vec::new();

        let mut naluStart = None;
        loop {
            match naluStart {
                None => match search_in(&self.buffer, &self.iden) {
                    Some(start) => {
                        naluStart = Some(start);
                        continue;
                    },
                    None => {
                        break;
                    },
                },
                Some(start) => {
                    let find_offset = start + self.iden.len();

                    let findNext = search_in(&self.buffer[find_offset..], &self.iden);

                    match findNext {
                        None => {
                            self.lastStart = Some(start);
                            break;
                        },
                        Some(next_step) => {
                            let next = find_offset + next_step;

                            let unitlen = next - start;

                            let s = start;
                            naluStart = Some(next);
                            v.push(&self.buffer[s..next]);
                        },
                    }
                },
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {

    use std::fs::{self};

    use super::*;
    #[test]
    fn test_case_ChunkerBy() {
        let mut chunker = ChunkerBy::new(&[0, 0, 0, 1]);
        {
            let c = chunker.chunk(&[0]);
            assert!(c.is_empty());
        }
        {
            let c = chunker.chunk(&[0, 0, 0, 1, 2, 3, 4, 0, 0, 0, 1]);
            assert_eq!(c.len(), 1);
            assert_eq!(c[0], [0, 0, 0, 1, 2, 3, 4].to_vec());
        }
        {
            let c = chunker.chunk(&[5, 0, 0, 0, 1]);
            assert_eq!(c.len(), 1);
            assert_eq!(c[0], [0, 0, 0, 1, 5].to_vec());
        }
        {
            let c = chunker.chunk(&[6, 0, 0]);
            assert!(c.is_empty());
        }
        {
            let c = chunker.chunk(&[6, 0, 0, 0]);
            assert!(c.is_empty());
        }
        {
            let c = chunker.chunk(&[1]);
            assert_eq!(c[0], [0, 0, 0, 1, 6, 0, 0, 6].to_vec());
        }
        {
            // continue 0.0.0.1
            let c = chunker.chunk(&[0, 0, 0, 1, 4, 4, 4, 0, 0, 0, 1, 5, 5, 0, 0, 0, 1]);
            assert_eq!(c.len(), 3);
            assert_eq!(c[0], [0, 0, 0, 1].to_vec());
            assert_eq!(c[1], [0, 0, 0, 1, 4, 4, 4].to_vec());
            assert_eq!(c[2], [0, 0, 0, 1, 5, 5].to_vec());
        }
    }
    #[test]
    fn test_case_ChunkerBy_file() {
        let content = fs::read(concat!(env!("H5SS_HOME"), "/tests/video-sample/nalu/ippp.264")).unwrap();
        let mut chunker = ChunkerBy::new(&[0x00, 0x00, 0x00, 0x01]);
        let cs = chunker.chunk(&content);

        assert_eq!(cs.len(), 5);
        assert_eq!(
            (cs[0].len(), cs[1].len(), cs[2].len(), cs[3].len(), cs[4].len()),
            (13, 8, 22373, 6366, 7659)
        );
        let z = chunker.flush();
        assert_eq!(chunker.flush().len(), 6711);
    }
    #[test]
    fn test_case_chunk_once() {
        let content = fs::read(concat!(env!("H5SS_HOME"), "/tests/video-sample/nalu/ippp.264")).unwrap();

        let cs = chunkBy_once(&content, &[0x00, 0x00, 0x00, 0x01]);

        assert_eq!(cs.len(), 6);
        assert_eq!(
            (
                cs[0].len(),
                cs[1].len(),
                cs[2].len(),
                cs[3].len(),
                cs[4].len(),
                cs[5].len()
            ),
            (13, 8, 22373, 6366, 7659, 6711)
        );
    }

    #[test]
    fn just_search_in() {
        assert_eq!(search_in(&[0, 1, 2, 3, 4, 1, 2, 3], &[1, 2, 3]), Some(1));
    }

    #[test]
    fn just_filter_in() {
        assert_eq!(filter_in(&[0, 1, 2, 3, 4, 1, 2, 3], &[1, 2, 3]), [1, 5]);
    }
}
