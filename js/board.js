/* jshint esversion: 6, browser: true, devel: true */
/* global toScreenX, toScreenY, Block, VISIBLE_HEIGHT, CUBE_MESH */

// Board grid standard is 14 rows by 10 columns

const BOARD = {
   get ROWS() {
      return 14;
   },
   get COLUMNS() {
      return 10;
   },
   get SPACING() {
      return 15;
   },
   get BLOCK_WIDTH() {
      return 30;
   },
   boardCenter: {
      x: VISIBLE_HEIGHT / 2,
      y: VISIBLE_HEIGHT / 2
   },
   fillRate: 2 * 100, // The rate at which cubes fill in units per second
   pendingFillTime: 0,
   dropQueue: function () {
      let boardFull = true;
      for (let i = 0; i < this.COLUMNS; i++)
         if (this.board[0][i] === null) {
            this.board[0][i] = this.queue[i];
            this.queue[i] = null;
            boardFull = false;
         }
      if (boardFull) this.pendingFillTime = 0;
   },
   fillQueue: function (delta) {
      this.pendingFillTime += delta;
      let shouldDrop = true;
      for (let i = 0; i < this.COLUMNS; i++) {

         if (!this.queue[i])
            this.queue[i] = new Block();
         let block = this.queue[i];

         if ((i === 0 || this.queue[i - 1].isFull()) && !block.isFull()) {
            let f = this.queue[i].fill;
            this.queue[i].fill += this.fillRate * this.pendingFillTime;
            this.pendingFillTime -= (this.queue[i].fill - f) / this.fillRate;
            shouldDrop = false;
         }
      }
      if (shouldDrop) {
         this.dropQueue();
      }
   },
   queue: [],
   board: [],
   dumpBlocks: [],

   init: function () {
      for (let r = 0; r < this.ROWS; r++) {
         this.board[r] = [];
         for (let c = 0; c < this.COLUMNS; c++) {
            this.board[r][c] = null;
         }
      }
      for (let i = 0; i < this.COLUMNS; i++)
         this.queue[i] = null;
   },

   /**
    * Does logic for the playing field
    *
    */

   logic: function (delta) {

      // Do board logic
      // Start from the bottom row and move up
      for (let r = this.ROWS - 1; r >= 0; r--) {
         for (let c = 0; c < this.COLUMNS; c++) {
            let block = this.board[r][c];
            if (block !== null)
               // If a block here exists, calculate physics on
               // this block
               block.blockLogic(delta, r, c);
            else if (r > 0) {
               // If this slot doesn't have a block and this
               // isn't the first row, then move the block
               // above this slot to this slot
               for (let i = r; i >= 0; i--) {
                  let blockAbove = this.board[i][c];
                  if (blockAbove !== null) {
                     this.board[r][c] = blockAbove;
                     this.board[i][c] = null;
                     break;
                  }
               }
            }
         }
      }

      // Queue logic
      this.fillQueue(delta);

      for (let i = 0; i < this.COLUMNS; i++)
         if (this.queue[i])
            this.queue[i].blockLogic(delta, -1, i);


      // Do trash block logic
      this.dumpBlocks.forEach((item, index, array) => {
         item.blockLogic(delta, item.row, item.col);
         if (item.gone)
            this.dumpBlocks.splice(index, 1);
      });

   },

   render: function (gl, programInfo) {
      this.renderBoardFrame(gl, programInfo);
      this.renderGrid(gl, programInfo);
      this.renderQueue(gl, programInfo);
      this.renderDumpBlocks(gl, programInfo);
   },

   renderQueue: function (gl, programInfo) {
      // Render the queue
      for (let i = 0; i < this.COLUMNS; i++) {
         let block = this.queue[i];
         if (block !== null)
            block.renderBlock(gl, programInfo);
      }
   },

   renderGrid: function (gl, programInfo) {
      for (let r = 0; r < this.ROWS; r++) {
         for (let c = 0; c < this.COLUMNS; c++) {
            let block = this.board[r][c];
            if (block !== null)
               block.renderBlock(gl, programInfo);
         }
      }
   },

   renderDumpBlocks: function (gl, programInfo) {
      // Render any falling blocks
      this.dumpBlocks.forEach((item, index, array) => {
         item.renderBlock(gl, programInfo);
      });
   },

   renderBoardFrame: function (gl, programInfo) {
      //      CUBE_MESH.render(gl, );
   },

   removeBlockFromBoard: function (row, col) {
      this.dumpBlocks.push(this.board[row][col]);
      this.board[row][col] = null;
   },

   getBoardX: function (col) {
      return this.boardCenter.x - this.getBoardWidth() / 2 + (col * (this.BLOCK_WIDTH + this.SPACING));
   },
   getBoardY: function (row) {
      let top = this.boardCenter.y - this.getBoardHeight() / 2;
      if (row == -1) return top;
      return top +
         // Add a row and spacing to accomodate the queue
         (row + 1) * (this.BLOCK_WIDTH + this.SPACING) +
         this.SPACING;
   },
   getBoardWidth: function () {
      return this.COLUMNS * (this.BLOCK_WIDTH + this.SPACING) - this.SPACING;
   },
   getBoardHeight: function () {
      // Here we include the block queue at the top, which has
      // double the spacing below it.
      return (this.ROWS + 1) * (this.BLOCK_WIDTH + this.SPACING);
   },

   removeIfMatchingNeighbors(row, col) {
      let block = this.board[row][col];
      if (!block)
         return;
      let blockInQuestion;
      let hadMatch = false;
      if (row > 0 && this.matchWithNeighbor(row, col, row - 1, col))
         hadMatch = true;
      if (row < this.ROWS - 1 && this.matchWithNeighbor(row, col, row + 1, col))
         hadMatch = true;

      if (col > 0 && this.matchWithNeighbor(row, col, row, col - 1))
         hadMatch = true;
      if (col < this.COLUMNS - 1 && this.matchWithNeighbor(row, col, row, col + 1))
         hadMatch = true;

      if (hadMatch)
         this.removeBlockFromBoard(row, col);
   },

   matchWithNeighbor: function (blockToRemoveRow, blockToRemoveColumn, neighborRow, neighborColumn) {
      let blockToRemove = this.board[blockToRemoveRow][blockToRemoveColumn];
      let neighbor = this.board[neighborRow][neighborColumn];

      if (!blockToRemove || !neighbor) return false;

      if (neighbor.color === blockToRemove.color) {
         blockToRemove.fall();
         if (!neighbor.falling)
            this.removeIfMatchingNeighbors(neighborRow, neighborColumn);
         return true;
      }
      return false;
   },

   onClick: function (event) {
      let mx = toScreenX(event.clientX);
      let my = toScreenY(event.clientY);
      let slotSize = this.BLOCK_WIDTH + this.SPACING;
      let row = Math.floor((my - this.boardCenter.y + (this.getBoardHeight() / 2) - this.SPACING) / slotSize - 1);
      let col = Math.floor((mx - this.boardCenter.x + (this.getBoardWidth() / 2)) / slotSize);
      this.removeIfMatchingNeighbors(row, col);
   }
};
