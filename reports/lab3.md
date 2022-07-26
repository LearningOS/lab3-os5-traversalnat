1. 实际上 p2 执行，由于溢出 p2.pass == 4 < p1.pass
2. 当 `PASS_MAX` 为 BigStride / 2, 对应的进程会等待其他进程执行直到 BigStride / 2 为最小的 pass 时，以此推理可得 `PASS_MAX - PASS_MIN <= BigStride / 2` 

3. 代码 
    ```rust
    use core::cmp::Ordering;

    struct Pass(u8);

    pub const BIG_STRIDE: u8 = u8::MAX;

    impl PartialOrd for Pass {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let difference = if self.0 > other.0 {
                self.0 - other.0
            } else {
                other.0 - self.0
            };

            if difference > BIG_STRIDE / 2 {
                Some(self.0.cmp(&other.0).reverse())
            } else {
                Some(self.0.cmp(&other.0))
            }
        }
    }

    impl PartialEq for Pass {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    ```
