/* jshint esversion: 6, browser: true, devel: true */

const gameLogic = (delta) => {
   factoriesLogic(delta);
   Board.logic(delta);
};

const renderGame = (delta, gl, programInfo, matrices, ctx2d) => {
   if (yInter < 1) {
      yInter += delta / yOffsetAnimateDuration;
      if (yInter > 1)
         yInter = 1;
      globalYOffset = yStart + quintEaseOut(yInter) * (yEnd - yStart);

   }
   // Set the camera offset
   setCameraOffset(0, globalYOffset, matrices.world, programInfo.uniformLocations.worldMatrix);
   ctx2d.translate(0, toBrowserY(globalYOffset));

   // Main stage
   renderMainStage(delta, gl, programInfo, ctx2d);

   // Render upper stage
   renderUpperStage(currentUpperStageMenu, delta, gl, programInfo, ctx2d);
};

const renderMainStage = (delta, gl, programInfo, ctx2d) => {
   Board.render(gl, programInfo);
   renderScoreboard(delta, gl, programInfo, ctx2d);
   renderStatusBar(delta, gl, programInfo, ctx2d);
};

const renderUpperStage = (stageMenu, delta, gl, programInfo, ctx2d) => {
   ctx2d.font = toBrowserH(DIALOG_TITLE_TEXT_HEIGHT) + 'px New Cicle Fina';
   ctx2d.fillStyle = 'black';
   ctx2d.textBaseline = 'middle';
   ctx2d.textBaseline = 'center';
   ctx2d.fillText(stageMenu, toBrowserX(getStatusBarX() + getStatusBarWidth() / 2), toBrowserY(getStatusBarHeight() / 2 - VISIBLE_HEIGHT));
   switch (stageMenu) {
      case StageMenu.FACTORIES:
         renderFactoryMenu(delta, gl, programInfo, ctx2d);
         break;
      case StageMenu.UPGRADES:
         break;
      case StageMenu.STATS:
         renderStats(ctx2d);
         break;
      case StageMenu.ACHIEVEMENTS:
         break;
   }
   backToBoardButton.render(delta, gl, programInfo, ctx2d);
};

let scoreboardFadeInter = 0;
let scoreboardFadeDuration = 0.5;

const renderScoreboard = (delta, gl, programInfo, ctx2d) => {
   // Render the block
   CubeMesh.setColor(COLOR_BLUE, gl, programInfo);
   let x = Board.boardCenter.x - Board.width / 2 - Board.GRID_PADDING - Board.FRAME_THICKNESS;
   let y = Board.boardCenter.y + Board.height / 2 + Board.GRID_PADDING + Board.FRAME_THICKNESS * 2;
   let w = Board.width + Board.FRAME_THICKNESS * 2 + Board.GRID_PADDING * 2;
   let h = VISIBLE_HEIGHT - y;
   CubeMesh.render(gl, x, y, 0, w, h, Board.BLOCK_WIDTH);

   // Set the text color //
   // If there are falling blocks from the board
   if (Board.dumpBlocks[0]) {
      if (scoreboardFadeInter < 1) {
         scoreboardFadeInter += delta / scoreboardFadeDuration;
         if (scoreboardFadeInter > 1) scoreboardFadeInter = 1;
      }
   } else {
      if (scoreboardFadeInter > 0) {
         scoreboardFadeInter -= delta / scoreboardFadeDuration;
         if (scoreboardFadeInter < 0) scoreboardFadeInter = 0;
      }
   }
   ctx2d.fillStyle = 'rgba(255, 255, 255,' + (1 - cubicEaseIn(scoreboardFadeInter) * 0.5) + ')';

   // Get fonts
   let textHeight = 50;
   let monospaceFont = toBrowserY(72) + 'px Digital-7';
   let cicleFont = toBrowserY(35) + 'px New Cicle Fina';

   let blocksTextX = toBrowserX(x + w - Board.FRAME_THICKNESS);
   let textY = toBrowserH(y + h / 2 + textHeight / 2);

   ctx2d.textBaseline = 'alphabetic';
   ctx2d.textAlign = 'right';

   ctx2d.font = cicleFont;
   ctx2d.fillText('blocks', blocksTextX, textY);

   let blocksTextWidth = ctx2d.measureText(' blocks').width;

   let amountText = Math.floor(Data.currentBlocks);
   ctx2d.font = monospaceFont;
   ctx2d.fillText(amountText, blocksTextX - blocksTextWidth, textY);
};

const getStatusBarX = () => {
   return VISIBLE_WIDTH / 2 - getStatusBarWidth() / 2;
};

const getStatusBarWidth = () => {
   return Board.width + Board.FRAME_THICKNESS * 2 + Board.GRID_PADDING * 2;
};

const getStatusBarHeight = () => {
   return Board.boardCenter.y - Board.height / 2 - Board.FRAME_THICKNESS * 3;
};

let globalYOffset = 0;

const Stage = {
   UPPER: 3,
   MAIN: 2,
   LOWER: 1
};

const StageMenu = {
   FACTORIES: 'Factories',
   UPGRADES: 'Upgrades',
   STATS: 'Statistics',
   ACHIEVEMENTS: 'Achievements',
   SETTINGS: 'Settings'
};

let currentStage = Stage.MAIN;
let currentUpperStageMenu;

const openUpperStage = (menu) => {
   currentUpperStageMenu = menu;
   yStart = globalYOffset;
   yEnd = VISIBLE_HEIGHT;
   yInter = 0;

   backToBoardButton.y = -getStatusBarHeight() / 2;
   backToBoardButton.text = 'keyboard_arrow_down';
};

const openLowerStage = menu => {
   currentLowerStageMenu = menu;
   yStart = globalYOffset;
   yEnd = -VISIBLE_HEIGHT;
   yInter = 0;

   backToBoardButton.y = VISIBLE_HEIGHT;
   backToBoardButton.text = 'keyboard_arrow_up';
};

const goBackToBoard = () => {
   yStart = globalYOffset;
   yEnd = 0;
   yInter = 0;
};

let yStart = 0;
let yEnd = 0;
let yInter = 0;
let yOffsetAnimateDuration = 0.5;

const initGame = () => {
   let x = getStatusBarX();
   let y = 0;
   let w = getStatusBarWidth();
   let h = getStatusBarHeight();

   factoriesButton = new Button(x, y, w, h / 2, COLOR_ORANGE, 'Factories', function () {
      openUpperStage(StageMenu.FACTORIES);
   });
   statsButton = new Button(x, y + h / 2, w - h / 2, h / 2, COLOR_BLUE, 'Stats', function () {
      openUpperStage(StageMenu.STATS);
   });
   settingsButton = new Button(x + w - h / 2, y + h / 2, h / 2, h / 2, COLOR_GREEN, 'settings', function () {
      openLowerStage(StageMenu.SETTINGS);
   });
   settingsButton.typeface = "Material Icons";
   settingsButton.fontSize = 36;

   backToBoardButton = new Button(x, y - h / 2, w, h / 2, COLOR_RED, 'keyboard arrow down', function () {
      goBackToBoard();
   });
   backToBoardButton.typeface = "Material Icons";
   backToBoardButton.fontSize = 36;
};

let factoriesButton;
let statsButton;
let settingsButton;
const renderStatusBar = (delta, gl, programInfo, ctx2d) => {
   factoriesButton.render(delta, gl, programInfo, ctx2d);
   statsButton.render(delta, gl, programInfo, ctx2d);
   settingsButton.render(delta, gl, programInfo, ctx2d);
};