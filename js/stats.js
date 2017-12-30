/* jshint esversion: 6, devel: true, browser: true */

const Stats = {
   get lifetimeBlocksCollected() {
      return this.lifetimeBlocksProducedByFactories + Data.lifetimeBlocksByColor.red + Data.lifetimeBlocksByColor.orange + Data.lifetimeBlocksByColor.green + Data.lifetimeBlocksByColor.blue;
   },
   get lifetimeBlocksProducedByFactories() {
      let total = 0;
      for (let prop in factories) {
         total += factories[prop].totalBlocksProduced;
      }
      return total;
   },
   get favoriteColor() {
      let max = Math.max(Data.lifetimeBlocksByColor.red, Math.max(Data.lifetimeBlocksByColor.orange, Math.max(Data.lifetimeBlocksByColor.green, Math.max(Data.lifetimeBlocksByColor.blue, Math.max(Data.lifetimeBlocksByColor.toxic, Data.lifetimeBlocksByColor.golden)))));
      switch (max) {
         case Data.lifetimeBlocksByColor.red:
            return "Red";
         case Data.lifetimeBlocksByColor.orange:
            return "Orange";
         case Data.lifetimeBlocksByColor.green:
            return "Green";
         case Data.lifetimeBlocksByColor.blue:
            return "Blue";
         case Data.lifetimeBlocksByColor.toxic:
            return "Toxic blocks";
         case Data.lifetimeBlocksByColor.golden:
            return "Golden blocks";
      }
   }
};

/**
 * @returns the y-coordinate where the next statistic can be rendered.
 * @param {string} statName
 * @param {string} statValue
 * @param {CanvasRenderingContext2D} ctx2d
 * @param {number} textY
 * @param {number} maxWidth
 */
const renderBigStat = (statName, statValue, ctx2d, textY, maxWidth) => {
   ctx2d.fillStyle = 'black';
   ctx2d.textBaseline = 'top';
   ctx2d.textAlign = 'center';

   let bigFontSize = UI_SANS_TEXT_HEIGHT * 1.25;
   let bigSansSerif = toBrowserH(bigFontSize) + 'px sans-serif';
   ctx2d.font = bigSansSerif;
   ctx2d.fillText(statName, toBrowserX(VISIBLE_WIDTH / 2), toBrowserY(textY));

   textY += bigFontSize;
   ctx2d.font = toBrowserH(75) + 'px Digital-7';
   ctx2d.fillText(statValue, toBrowserX(VISIBLE_WIDTH / 2), toBrowserY(textY), maxWidth);

   return textY + 75 + UI_PADDING;
};

const renderStats = ctx2d => {
   let maxWidth = toBrowserW(VISIBLE_WIDTH - UI_PADDING * 2);
   let textY = -VISIBLE_HEIGHT + getStatusBarHeight() + UI_PADDING;

   textY = renderBigStat('Lifetime blocks produced or collected', Math.floor(Stats.lifetimeBlocksCollected).toLocaleString(), ctx2d, textY, maxWidth);
   textY = renderBigStat('Lifetime pollutants produced', Math.floor(Data.lifetimePollution).toLocaleString(), ctx2d, textY, maxWidth);

   let numStats = 5;
   let textX = toBrowserX(UI_PADDING);
   ctx2d.textAlign = "left";
   ctx2d.font = getSansFont();
   for (let i = 0; i < numStats; i++) {
      switch (i) {
         case 0:
            let total = 0;
            for (let prop in Data.lifetimeBlocksByColor)
               total += Data.lifetimeBlocksByColor[prop];

            ctx2d.fillText(total.toLocaleString() + " total blocks collected by hand", textX, toBrowserY(textY));
            break;
         case 1:
            ctx2d.fillText("Favorite block color: " + Stats.favoriteColor, textX, toBrowserY(textY));
            break;
      }
      textY += UI_SANS_TEXT_HEIGHT * 1.15;
   }
};