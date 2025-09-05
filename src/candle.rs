#[cfg(test)]
mod test {
    use candle_core::{DType, Device, Module, Tensor};
    use candle_nn::Embedding;

    #[test]
    fn test() {
        let device = Device::Cpu;

        let a = Tensor::randn(0f32, 1., (2, 3), &device).unwrap();
        let b = Tensor::randn(0f32, 1., (3, 4), &device).unwrap();

        let c = a.matmul(&b).unwrap();
        println!("{c}");
    }

    #[test]
    fn embedding() -> anyhow::Result<()> {
        let dev = Device::Cpu;
        let (vocab_size, hidden_size) = (32000, 4096);

        // 已有的权重矩阵（例如从权重文件加载），形状 [V, D]
        let weight = Tensor::zeros((vocab_size, hidden_size), DType::F32, &dev)?;

        // 构造 Embedding
        let emb = Embedding::new(weight, hidden_size);

        // 模拟输入 token ids，形状 [B, T]
        let next_ids: Vec<u32> = vec![42, 314, 2024]; // [batch]
        let len = next_ids.len();
        // 构造形状为 [batch] 的索引张量
        let input_ids = Tensor::from_vec(next_ids, (len,), &dev)?;

        // 前向：能够通过查表得到对应的 embedding
        let xs = emb.forward(&input_ids)?;
        println!("{:?}", xs.dims()); // [2, 5, 4096]
        println!("{}", xs.get(0)?);

        Ok(())
    }
}
