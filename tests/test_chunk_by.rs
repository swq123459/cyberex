#[allow(non_snake_case)]
#[cfg(test)]
mod tests {

    use cyberex::{
        buf_pro::{chunk_by::*, search::*},
        xfs::cargo_path::get_project_root_path,
    };
    use std::fs::{self};

    #[test]
    fn test_case_chunker_by() {
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
    fn test_case_chunker_by_file() {
        let content = fs::read(
            get_project_root_path()
                .unwrap()
                .join("tests/video-sample/nalu/ippp.264"),
        )
        .unwrap();
        let mut chunker = ChunkerBy::new(&[0x00, 0x00, 0x00, 0x01]);
        let cs = chunker.chunk(&content);

        assert_eq!(cs.len(), 5);
        assert_eq!(
            (cs[0].len(), cs[1].len(), cs[2].len(), cs[3].len(), cs[4].len()),
            (13, 8, 22373, 6366, 7659)
        );
        let _ = chunker.flush();
        assert_eq!(chunker.flush().len(), 6711);
    }
    #[test]
    fn test_case_chunk_once() {
        let content = fs::read(
            get_project_root_path()
                .unwrap()
                .join("tests/video-sample/nalu/ippp.264"),
        )
        .unwrap();

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
    fn test_chunkByIf_once() {
        let vsom = [
            vec![0x00, 0x00, 0x01, 0xBA, 0x11],
            vec![0x00, 0x00, 0x01, 0xE0, 0x11, 0x11, 0x11, 0x11, 0x11],
            vec![0x00, 0x00, 0x01, 0xC0, 0x11, 0x11, 0x11],
            vec![0x00, 0x00, 0x01, 0xE0, 0x11, 0x11, 0x11, 0x11, 0x11],
        ];
        let v = vsom.clone().into_iter().flatten().collect::<Vec<_>>();

        let k = chunkByIf_once(&v, 4, |i, w| {
            w == [0x00, 0x00, 0x01, 0xE0] || w == [0x00, 0x00, 0x01, 0xC0] || w == [0x00, 0x00, 0x01, 0xBD] || i == 0
        });
        for (i, s) in k.into_iter().enumerate() {
            assert_eq!(s, vsom[i]);
        }
    }

    #[test]
    fn just_search_in() {
        assert_eq!(search_in(&[0, 1, 2, 3, 4, 1, 2, 3], &[1, 2, 3]), Some(1));
    }

    #[test]
    fn just_filter_in() {
        assert_eq!(filter_in(&[0, 1, 2, 3, 4, 1, 2, 3], &[1, 2, 3]), [1, 5]);
    }
    #[test]
    fn just_filter_in_if() {
        assert_eq!(
            filter_in_if(&[0, 1, 2, 3, 4, 1, 2, 4], 3, |_, w| {
                w == [1, 2, 3] || w == [1, 2, 4]
            }),
            [1, 5]
        );
        assert_eq!(
            filter_in_if(&[0, 1, 2, 3, 4, 1, 2, 4], 3, |i, w| {
                w == [1, 2, 3] || w == [1, 2, 4] || i == 0
            }),
            [0, 1, 5]
        );
        assert_eq!(
            filter_in_if(&[0, 1, 2, 3, 4, 1, 2, 4], 3, |_, w| {
                w == [1, 1, 1] || w == [2, 2, 2]
            }),
            []
        );
    }
}
