#[cfg(test)]
mod tests {
    use cyberex::buf_pro::chunk::Chunker;

    #[test]
    fn test_case_chunk() {
        let mut chunker = Chunker::<i32>::new(2);
        {
            let c = chunker.chunk([].as_ref());
            assert_eq!(c.len(), 0);

        }
        {
            let c = chunker.chunk([1, 2, 3, 4].as_ref());

            assert_eq!(c.len(), 2);
            assert_eq!(c, [[1, 2], [3, 4]].to_vec());
        }
        {
            let c = chunker.chunk([5].as_ref());

            assert_eq!(c.len(), 0);
        }
        {
            let c = chunker.chunk([6].as_ref());

            assert_eq!(c.len(), 1);
            assert_eq!(c, [[5, 6]].to_vec());
        }
        {
            let c = chunker.chunk([7, 8, 9].as_ref());

            assert_eq!(c.len(), 1);
            assert_eq!(c, [[7, 8]].to_vec());
        }
        {
            let c = chunker.chunk([10, 11, 12].as_ref());

            assert_eq!(c.len(), 2);
            assert_eq!(c, [[9, 10], [11, 12]].to_vec());
        }
    }
}
