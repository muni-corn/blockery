/* jshint esversion: 6 */
/* global BigNumber, BOARD, localStorage */

const DATA = {
   currentBlocks: new BigNumber(0),
   lifetimeBlocks: {
      red: new BigNumber(0),
      orange: new BigNumber(0),
      green: new BigNumber(0),
      blue: new BigNumber(0)
   },
   lifetimeClicks: {
      failed: 0,
      successful: 0
   },
   boardCode: undefined,
   load: function () {
      let version = localStorage.getItem(KEY_VERSION);
      if (!version) return;

      // Loading procedure for version 1
      this.currentBlocks = new BigNumber(localStorage.getItem(KEY_CURRENT_BLOCKS));

      this.boardCode = localStorage.getItem(KEY_GRID);

      this.lifetimeBlocks.red = new BigNumber(localStorage.getItem(KEY_RED));
      this.lifetimeBlocks.orange = new BigNumber(localStorage.getItem(KEY_ORANGE));
      this.lifetimeBlocks.green = new BigNumber(localStorage.getItem(KEY_GREEN));
      this.lifetimeBlocks.blue = new BigNumber(localStorage.getItem(KEY_BLUE));

      this.lifetimeClicks.successful = new BigNumber(localStorage.getItem(KEY_SUCCESSFUL_CLICKS));
      this.lifetimeClicks.failed = new BigNumber(localStorage.getItem(KEY_FAILED_CLICKS));

   },
   save: function () {
      localStorage.setItem(KEY_VERSION, DATABASE_VERSION);

      // Saving procedure for version 1
      localStorage.setItem(KEY_CURRENT_BLOCKS, this.currentBlocks);

      localStorage.setItem(KEY_GRID, BOARD.getGridCode());

      localStorage.setItem(KEY_RED, this.lifetimeBlocks.red);
      localStorage.setItem(KEY_ORANGE, this.lifetimeBlocks.orange);
      localStorage.setItem(KEY_GREEN, this.lifetimeBlocks.green);
      localStorage.setItem(KEY_BLUE, this.lifetimeBlocks.blue);

      localStorage.setItem(KEY_SUCCESSFUL_CLICKS, this.lifetimeClicks.successful);
      localStorage.setItem(KEY_FAILED_CLICKS, this.lifetimeClicks.failed);

      console.log(localStorage.getItem(KEY_GRID));

   }
};

const KEY_VERSION = 'version';
const KEY_GRID = 'grid';
const KEY_CURRENT_BLOCKS = 'currentBlocks';
const KEY_RED = 'red';
const KEY_ORANGE = 'orange';
const KEY_GREEN = 'green';
const KEY_BLUE = 'blue';
const KEY_SUCCESSFUL_CLICKS = 'successfulClicks';
const KEY_FAILED_CLICKS = 'failedClicks';

const DATABASE_VERSION = 1;
