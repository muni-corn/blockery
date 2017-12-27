/* jshint esversion: 6, browser: true, devel: true */

const LISTENER_BLOCK_COUNT_FLAG = 0x00000001;
const LISTENER_FACTORY_PURCHASE_FLAG = 0x00000002;

class Unlockable {
   constructor(bit, name, listeners, onUnlockFunction, conditionFunction) {
      this.bit = bit;
      this.name = name;
      const checkForUnlock = function (flag, ...args) {
         if (conditionFunction(flag, args))
            onUnlockFunction(flag, args);
      };
      if (listeners & LISTENER_BLOCK_COUNT_FLAG)
         Listeners.blockCountListeners.push({
            onBlockCount: function (count) {
               checkForUnlock(LISTENER_BLOCK_COUNT_FLAG, count);
         }
      });
      if (listeners & LISTENER_FACTORY_PURCHASE_FLAG) {
         Listeners.blockCountListeners.push({
            onFactoryPurchase: function (factoryCode, count) {
               checkForUnlock(LISTENER_FACTORY_PURCHASE_FLAG, factoryCode, count);
            }
         });
      }
   }
}