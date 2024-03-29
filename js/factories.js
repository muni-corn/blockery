/* jshint esversion: 6 */
/* global VISIBLE_HEIGHT, mouseListeners, Data, Listeners, UI_PADDING, ProgressButton, ImageButton, Board, COLOR_RED, COLOR_ORANGE, COLOR_GREEN, COLOR_BLUE, COLOR_POISON, getStatusBarX,getStatusBarWidth, UI_SANS_TEXT_HEIGHT, getStatusBarHeight, Button, goBackToBoard, CUBE_MESH, toBrowserX, toBrowserY, DIALOG_TITLE_TEXT_HEIGHT, toBrowserH, getSansFont*/

const COLOR_DISABLED_PURCHASE = 0x002caf;
const COLOR_DARK_GREEN = 0x009c3c;

let globalBlockRateMultiplier = 1,
   globalPollutionMultiplier = 1,
   globalCapacityMultiplier = 1,
   globalEmptyRateMultiplier = 1,
   globalPriceMultiplier = 1;

const PRICE_INCREASE = 1.25; // 125%
const BASE_EMPTY_RATE = 100;

const PURCHASE_BUTTON_SIZE = Board.BLOCK_WIDTH * 3;
const STORAGE_BUTTON_HEIGHT = Board.BLOCK_WIDTH * 1.5;

let factoriesUnlocked = 0;
const FACTORIES_PER_PAGE = 3;
let currentFactoryPage = 0;
let factoriesMenuYOffset = 0;

class Factory {
   constructor(factoryCode, index, name, imgSrc, basePrice, blockRate, pollutionRate, capacity) {
      this.factoryCode = factoryCode;
      this.index = index;
      this.page = index / FACTORIES_PER_PAGE;
      this.name = name;
      this.imgSrc = imgSrc;
      this.basePrice = basePrice;
      this.baseProductionRate = blockRate;
      this.basePollutionRate = pollutionRate;
      this.baseCapacity = capacity;
      this.blocksHeld = 0;
      this.localBlockRateMultiplier = 1;
      this.localPollutionMultiplier = 1;
      this.localCapactiyMultiplier = 1;
      this.localPriceMultiplier = 1;
      this.amountOwned = 0;

      this.peek = false;
      this.available = false;

      this.totalPollutionProduced = 0;
      this.totalBlocksProduced = 0;

      this.progressButton = new ProgressButton(getStatusBarX(), 0, getStatusBarWidth(), STORAGE_BUTTON_HEIGHT, COLOR_GREEN, COLOR_DARK_GREEN, '', () => {
         this.empty();
      });
      this.progressButton.typeface = 'Digital-7';

      this.imageButton = new ImageButton(getStatusBarX(), 0, PURCHASE_BUTTON_SIZE, PURCHASE_BUTTON_SIZE, COLOR_BLUE, "img/unknown.png", () => {
         this.buy();
      });
      this.imageButton.disabledColor = COLOR_DISABLED_PURCHASE;

      this.wasHidden = true;
   }

   logic(delta) {
      this.produceBlocks(delta);
   }

   empty() {
      this.emptying = true;
   }

   emptyImmediately() {
      let emptiedBlocks = Math.floor(this.blocksHeld);
      Data.currentBlocks += emptiedBlocks;
      this.blocksHeld -= emptiedBlocks;
      this.emptying = false;
   }

   produceBlocks(delta) {
      if (this.emptying) {

         let emptiedBlocks = this.totalEmptyRate * delta;

         if (emptiedBlocks > this.blocksHeld) {
            // If we have emptied more blocks than this factory is holding,
            // set delta to the amount of time put into emptying nonexistent
            // blocks so that it can be used after this if statement to produce
            // blocks
            delta = (emptiedBlocks - Math.floor(this.blocksHeld)) / (this.totalEmptyRate);
            Data.currentBlocks += Math.floor(this.blocksHeld);
            this.blocksHeld -= Math.floor(this.blocksHeld);
            this.emptying = false;
         } else {

            // Otherwise, just empty the blocks as expected and return from this
            // function so that we do not produce blocks while we empty
            this.blocksHeld -= emptiedBlocks;
            Data.currentBlocks += emptiedBlocks;
            return;
         }

      }


      if (!globalBlockProductionEnabled)
         return;

      // Produce blocks and pollution
      if (this.blocksHeld < this.totalCapacity) {

         let newBlocks = this.totalBlockRate * delta;
         this.totalBlocksProduced += newBlocks;
         this.blocksHeld += newBlocks;
         if (this.blocksHeld > this.totalCapacity)
            this.blocksHeld = this.totalCapacity;

         let newPollution = this.totalPollutionRate * delta;
         Data.currentPollution += newPollution;
         this.totalPollutionProduced += newPollution;
      }
   }

   isAffordable(blocks) {
      return blocks >= this.price;
   }

   isBasePriceAffordable(blocks) {
      return blocks >= this.basePrice;
   }

   buy() {
      if (this.isAffordable(Data.currentBlocks)) {
         Data.currentBlocks -= this.price;
         this.amountOwned++;
         new Firework(this.imageButton.x + this.imageButton.w / 2, this.imageButton.y + this.imageButton.h / 2);
      }
   }

   get totalBlockRate() {
      return this.amountOwned * this.singularBlockRate;
   }

   get singularBlockRate() {
      return this.baseProductionRate * this.localBlockRateMultiplier * globalBlockRateMultiplier;
   }

   get totalCapacity() {
      return this.amountOwned * this.baseCapacity * this.localCapactiyMultiplier * globalCapacityMultiplier;
   }

   get totalPollutionRate() {
      return this.amountOwned * this.basePollutionRate * this.localPollutionMultiplier * globalPollutionMultiplier;
   }

   get totalEmptyRate() {
      return this.amountOwned * BASE_EMPTY_RATE * globalEmptyRateMultiplier;
   }

   get price() {
      return Math.floor(this.basePrice * Math.pow(PRICE_INCREASE, this.amountOwned) * this.localPriceMultiplier * globalPriceMultiplier);
   }

   /** Includes padding at the bottom. */
   static get infoCardHeight() {
      return PURCHASE_BUTTON_SIZE + STORAGE_BUTTON_HEIGHT + UI_PADDING * 2;
   }

   get visibleOnPage() {
      return Math.floor(this.index / FACTORIES_PER_PAGE) == currentFactoryPage;
   }

   renderOptions(delta, gl, programInfo, ctx2d, yOffset) {
      if (!this.visibleOnPage || this.index > factoriesUnlocked) {
         this.imageButton.enabled = false;
         this.progressButton.enabled = false;
         return;
      }

      let hidden = factoriesUnlocked === this.index;
      if (!hidden && this.wasHidden) {
         this.imageButton.imgSrc = this.imgSrc;
         this.wasHidden = hidden;
      }

      // Assign an easier variable for UI_PADDING
      let p = UI_PADDING;

      let statusBarHeight = getStatusBarHeight();
      let y = statusBarHeight + (getPageChangerButtonY() - statusBarHeight) / 2 - (Factory.infoCardHeight * FACTORIES_PER_PAGE - UI_PADDING * 2) / 2 + this.index % FACTORIES_PER_PAGE * Factory.infoCardHeight + yOffset;

      this.imageButton.enabled = Data.currentBlocks >= this.price && !hidden;
      this.imageButton.y = y;
      this.imageButton.text = (this.amountOwned > 0) ? this.amountOwned : 0;
      this.imageButton.render(delta, gl, programInfo, ctx2d);

      ctx2d.textAlign = 'left';
      ctx2d.textBaseline = 'top';
      ctx2d.fillStyle = 'black';

      // Header
      let textX = toBrowserX(this.imageButton.x + this.imageButton.w + UI_PADDING);
      let textY = toBrowserY(this.imageButton.y);
      ctx2d.font = toBrowserH(DIALOG_TITLE_TEXT_HEIGHT) + 'px New Cicle Fina';
      ctx2d.fillText(hidden ? 'Under construction' : this.name, textX, textY);

      // Info //
      ctx2d.font = getSansFont();
      ctx2d.textBaseline = 'alphabetical';
      textY = toBrowserY(this.imageButton.y + DIALOG_TITLE_TEXT_HEIGHT * 1.15);

      ctx2d.fillText('Costs ' + this.price.toLocaleString() + ' for +' + this.singularBlockRate.toLocaleString() + ' bps', textX, textY);

      // If this factory is owned...
      if (this.amountOwned > 0) {
         // ...declare 'full' if its capacity has been reached...
         if (this.blocksHeld === this.totalCapacity) {
            ctx2d.fillText('Full', textX, textY + toBrowserH(UI_SANS_TEXT_HEIGHT) * 1.15);
            // ...or, display how much time it will take until it is full (if not emptying)
            // or empty (if emptying)
         } else {
            // Get the time remaining...
            let timeLeft;
            if (this.emptying)
               // ...until empty
               timeLeft = this.blocksHeld / this.totalEmptyRate;
            else
               // ... until full
               timeLeft = (this.totalCapacity - this.blocksHeld) / this.totalBlockRate;

            // By default, time is measured in seconds but reduced to larger
            // time units if timeLeft is too large
            let timeUnit = 'seconds';

            if (timeLeft >= 3600 * 24 * 7) {
               timeLeft /= 3600 * 24 * 7;
               timeUnit = 'weeks';
            } else if (timeLeft >= 3600 * 24) {
               timeLeft /= 3600 * 24;
               timeUnit = 'days';
            } else if (timeLeft >= 3600) {
               timeLeft /= 3600;
               timeUnit = 'hours';
            } else if (timeLeft >= 60) {
               timeLeft /= 60;
               timeUnit = 'minutes';
            }

            if (timeUnit === 'seconds') {
               // round (up) to tenths of seconds
               timeLeft = (Math.ceil(timeLeft * 10) / 10).toFixed(1);
            } else {
               // or just ceiling everything else
               timeLeft = Math.ceil(timeLeft);
            }

            ctx2d.fillText((this.emptying ? 'Empty in ' : 'Full in ') + timeLeft + ' ' + timeUnit, textX, textY + toBrowserH(UI_SANS_TEXT_HEIGHT) * 1.15);
         }

         this.progressButton.enabled = !this.emptying;
         this.progressButton.progress = this.blocksHeld / this.totalCapacity;
         this.progressButton.text = Math.floor(this.blocksHeld) + ' / ' + this.totalCapacity;
         this.progressButton.y = y + this.imageButton.h;
         this.progressButton.render(delta, gl, programInfo, ctx2d);
      }
   }

   applyCode(version, code) {
      switch (version) {
         case 1:
            let split = code.split('|');
            this.amountOwned = Number.parseInt(split[0]);
            this.blocksHeld = Number.parseFloat(split[1]);
            this.totalBlocksProduced = Number.parseFloat(split[2]);
            this.totalPollutionProduced = Number.parseFloat(split[3]);
            break;
      }
   }

   get saveCode() {
      return `${this.amountOwned}|${this.blocksHeld}|${this.totalBlocksProduced}|${this.totalPollutionProduced}`;
   }
}

const FACTORY_SMIT = new Factory("smit", 0, 'Blocksmith', 'img/smit.png', 250, 0.5, 0.1, 50);
const FACTORY_COTT = new Factory("cott", 1, 'Cottage factory', 'img/cott.png', FACTORY_SMIT.basePrice * 12, 0.5 * 4, 0.1 * 5, 50 * 6);
const FACTORY_MINE = new Factory("mine", 2, 'Block mine', 'img/mine.png', FACTORY_COTT.basePrice * 12, 0.5 * 4 * 8, 0.1 * 5 * 10, 50 * 6 * 12);
const FACTORY_POWH = new Factory("powh", 3, 'Powerhouse', 'img/powh.png', FACTORY_MINE.basePrice * 12, 0.5 * 4 * 8 * 12, 0.1 * 5 * 10 * 15, 50 * 6 * 12 * 18);
const FACTORY_CLMK = new Factory("clmk", 4, 'Cloudmaker', 'img/clmk.png', FACTORY_POWH.basePrice * 12, 0.5 * 4 * 8 * 12 * 16, 0.1 * 5 * 10 * 15 * 20, 50 * 6 * 12 * 18 * 24);
const FACTORY_VOLC = new Factory("volc", 5, 'Block volcano', 'img/volc.png', FACTORY_CLMK.basePrice * 12, 0.5 * 4 * 8 * 12 * 16 * 20, 0.1 * 5 * 10 * 15 * 20 * 25, 50 * 6 * 12 * 18 * 24 * 30);
const FACTORY_MNFM = new Factory("mnfm", 6, 'Moon block farm', 'img/mnfm.png', FACTORY_VOLC.basePrice * 12, 0.5 * 4 * 8 * 12 * 16 * 20 * 24, 0.1 * 5 * 10 * 15 * 20 * 25 * 30, 50 * 6 * 12 * 18 * 24 * 30 * 36);
const FACTORY_PLSM = new Factory("plsm", 7, 'Planetary block storm', 'img/plsm.png', FACTORY_MNFM.basePrice * 12, 0.5 * 4 * 8 * 12 * 16 * 20 * 24 * 28, 0.1 * 5 * 10 * 15 * 20 * 25 * 30 * 35, 50 * 6 * 12 * 18 * 24 * 30 * 36 * 42);
const FACTORY_STAR = new Factory("star", 8, 'Star reactor', 'img/star.png', FACTORY_PLSM.basePrice * 12, 0.5 * 4 * 8 * 12 * 16 * 20 * 24 * 28 * 32, 0.1 * 5 * 10 * 15 * 20 * 25 * 30 * 35 * 40, 50 * 6 * 12 * 18 * 24 * 30 * 36 * 42 * 48);
const FACTORY_DMGT = new Factory("dmgt", 9, 'Interdimensional gateway', 'img/dmgt.png', FACTORY_STAR.basePrice * 12, 0.5 * 4 * 8 * 12 * 16 * 20 * 24 * 28 * 32 * 36, 0.1 * 5 * 10 * 15 * 20 * 25 * 30 * 35 * 40 * 45, 50 * 6 * 12 * 18 * 24 * 30 * 36 * 42 * 48 * 54);

// Excuse this mess
let factories = {
    smit: FACTORY_SMIT,
    cott: FACTORY_COTT,
    mine: FACTORY_MINE,
    powh: FACTORY_POWH,
    clmk: FACTORY_CLMK,
    volc: FACTORY_VOLC,
    mnfm: FACTORY_MNFM,
    plsm: FACTORY_PLSM,
    star: FACTORY_STAR,
    dmgt: FACTORY_DMGT,
   // The Everything Dimension?
};

const factoriesLogic = delta => {
   for (let prop in factories) {
      factories[prop].logic(delta);
   }
};

let upperStageBackButton;
let nextPageButton, previousPageButton;
const PAGE_CHANGER_BUTTON_WIDTH = 150;
const PAGE_CHANGER_BUTTON_HEIGHT = 50;

const getPageChangerButtonY = () => {
   return VISIBLE_HEIGHT - getStatusBarHeight() * 1.5 - UI_PADDING - PAGE_CHANGER_BUTTON_HEIGHT;
};

const getMaxPage = () => {
   return Math.floor(factoriesUnlocked / FACTORIES_PER_PAGE);
};

const renderFactoryMenu = (delta, gl, programInfo, ctx2d, yOffset) => {
   for (let prop in factories) {
      factories[prop].renderOptions(delta, gl, programInfo, ctx2d, yOffset);
   }
   if (previousPageButton)
      previousPageButton.render(delta, gl, programInfo, ctx2d, yOffset);

   if (nextPageButton)
      nextPageButton.render(delta, gl, programInfo, ctx2d, yOffset);


   ctx2d.font = toBrowserH(UI_SANS_TEXT_HEIGHT * 1.5) + "px New Cicle Fina";
   ctx2d.fillStyle = "black";
   ctx2d.textBaseline = "middle";
   ctx2d.textAlign = "center";
   ctx2d.fillText((currentFactoryPage + 1) + " / " + (getMaxPage() + 1), toBrowserX(VISIBLE_WIDTH / 2), toBrowserY(getPageChangerButtonY() + nextPageButton.h / 2 + yOffset));

   renderFactoryMenuScoreboard(gl, programInfo, ctx2d, yOffset);
};

const renderFactoryMenuScoreboard = (gl, programInfo, ctx2d, yOffset) => {
   // Render the block
   CubeMesh.setColor(COLOR_BLUE, gl, programInfo);
   let h = getStatusBarHeight();
   let w = getStatusBarWidth();
   let x = getStatusBarX();
   let y = VISIBLE_HEIGHT - h * 1.5 + yOffset;
   CubeMesh.render(gl, x, y, 0, w, h, Board.BLOCK_WIDTH);

   // Set the text color
   ctx2d.fillStyle = 'white';

   // Get fonts
   let textHeight = 50;
   let monospaceFont = toBrowserH(textHeight) + 'px Digital-7';
   let cicleFont = toBrowserH(textHeight / 2) + 'px New Cicle Fina';

   let blocksTextX = toBrowserX(x + w - Board.FRAME_THICKNESS);
   let textY = toBrowserY(y + h / 2 + textHeight / 2);

   ctx2d.font = cicleFont;
   let rightIndent = Math.max(ctx2d.measureText(' blocks').width, ctx2d.measureText(' stored').width);
   ctx2d.textBaseline = 'center';

   ctx2d.textAlign = 'left';
   ctx2d.fillText(' blocks', blocksTextX - rightIndent, toBrowserY(y + h / 3));
   ctx2d.fillStyle = "rgba(255, 255, 255, 0.75)";
   ctx2d.fillText(' stored', blocksTextX - rightIndent, toBrowserY(y + h * 2 / 3));

   ctx2d.textAlign = 'right';
   let amountText = Math.floor(Data.currentBlocks);
   let totalStoredBlocks = 0;
   for (let prop in factories)
      totalStoredBlocks += factories[prop].blocksHeld;
   ctx2d.font = monospaceFont;
   ctx2d.fillText('+' + Math.floor(totalStoredBlocks), blocksTextX - rightIndent, toBrowserY(y + (h * 2 / 3)));
   ctx2d.fillStyle = "white";
   ctx2d.fillText(amountText, blocksTextX - rightIndent, toBrowserY(y + (h / 3)));
};

const checkPageButtons = () => {
   if (!nextPageButton) {
      nextPageButton = new Button(getStatusBarX() + getStatusBarWidth() - PAGE_CHANGER_BUTTON_HEIGHT, getPageChangerButtonY(), PAGE_CHANGER_BUTTON_HEIGHT, PAGE_CHANGER_BUTTON_HEIGHT, COLOR_ORANGE, "keyboard_arrow_right", function () {
         if (currentFactoryPage < getMaxPage())
            currentFactoryPage++;
         checkPageButtons();
      });
      nextPageButton.typeface = "Material Icons";
      nextPageButton.fontSize = 36;
   }

   if (!previousPageButton) {
      previousPageButton = new Button(getStatusBarX(), getPageChangerButtonY(), PAGE_CHANGER_BUTTON_HEIGHT, PAGE_CHANGER_BUTTON_HEIGHT, COLOR_ORANGE, "keyboard_arrow_left", function () {
         if (currentFactoryPage > 0)
            currentFactoryPage--;
         checkPageButtons();
      });
      previousPageButton.typeface = "Material Icons";
      previousPageButton.fontSize = 36;
   }

   nextPageButton.enabled = currentFactoryPage < getMaxPage();
   previousPageButton.enabled = currentFactoryPage > 0;
};

// Check to see if new pages have become available as blocks are collected
Listeners.blockCountListeners.push({
   onBlockCount: function (blocks) {
      let numFactories = Object.keys(factories).length;
      if (factoriesUnlocked < numFactories) {
         let factory;
         let factoryIsAffordable;
         do {
            factory = factories[Object.keys(factories)[factoriesUnlocked]];
            if (!factory)
               return;
            factoryIsAffordable = factory.isBasePriceAffordable(blocks);
            if (factoryIsAffordable)
               factoriesUnlocked++;
         } while (factoryIsAffordable && numFactories);
      }
      checkPageButtons();
   }
});
