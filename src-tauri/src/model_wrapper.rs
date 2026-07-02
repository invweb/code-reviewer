use candle_core::Tensor;
use candle_transformers::models::quantized_llama as llama;

pub trait QuantizedModel {
    fn forward(&self, tokens: &[u32], start_pos: usize) -> candle_core::Result<Tensor>;
    fn context_size(&self) -> usize;
}

impl QuantizedModel for llama::Model {
    fn forward(&self, tokens: &[u32], start_pos: usize) -> candle_core::Result<Tensor> {
        self.forward(tokens, start_pos)
    }
    fn context_size(&self) -> usize {
        self.context_size()
    }
}
