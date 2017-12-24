/* jshint esversion: 6, browser: true, devel: true */

const Listeners = {
   blockCountListeners: []

};

const invokeBlockCountListeners = count => {
   Listeners.blockCountListeners.forEach(listener => {
      listener.onBlockCount(count);
   });
};

