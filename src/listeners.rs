pub trait BlockCountListener {
    fn on_block_count(count: u128);
}

pub trait FactoryPurchaseListener {
    fn on_factory_purchase(factory_type: FactoryType, count: u32);
}

pub struct Listeners {
   block_count_listeners: Vec<Box<dyn BlockCountListener>>,
   factory_purchase_listeners: Vec<Box<dyn FactoryPurchaseListener>>,
}

impl Listeners {
   fn invoke_block_count_listeners(&self, count: u128) {
      for listener in self.block_count_listeners {
         listener.onBlockCount(count);
      }
   }

   fn invoke_factory_purchase_listeners(&self, factory_code: &str, count: u32) {
      for listener in self.factory_purchase_listeners {
         listener.onFactoryPurchase(factory_code, count);
      }
   }
}
