use super::chunk_by::ChunkerBy;
pub struct SpliterBy<T> {
    chunker: ChunkerBy<T>,
    cache: Vec<T>,
    done: bool,
}

impl<T: PartialEq + Clone> SpliterBy<T> {
    pub fn new(iden: &[T]) -> Self {
        Self {
            chunker: ChunkerBy::new(iden),
            cache: Vec::new(),
            done: false,
        }
    }

    pub fn chunk<'a>(&'a mut self, data_input: &[T]) -> Vec<&'a [T]> {
        let chunker = &mut self.chunker;

        if self.done {
            return chunker.chunk(data_input);
        }

        let mut v = Vec::new();
        if self.cache.len() >= chunker.iden().len() {
            let iden = chunker.iden().to_owned();
            if !self.cache.starts_with(&iden) {
                let mut input = Vec::new();
                input.extend_from_slice(&iden);
                input.extend_from_slice(&self.cache);
                input.extend_from_slice(data_input);
                let vs = &chunker.chunk(&input);
                if !vs.is_empty() {
                    for (index, e) in vs.iter().enumerate() {
                        if index == 0 {
                            let real_v1: &[T] = &e[iden.len()..];
                            v.push(real_v1);
                        } else {
                            v.push(e)
                        }
                    }
                }
            } else {
                v.extend_from_slice(&chunker.chunk(&self.cache));
            }
            self.done = true;
        } else {
            self.cache.extend_from_slice(data_input);
        }
        v
    }
}
