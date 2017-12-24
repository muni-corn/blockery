/* jshint esversion: 6, browser: true, devel: true */

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

    // Main level
    Board.render(gl, programInfo);
    renderScoreboard(delta, gl, programInfo, ctx2d);
    renderStatusBar(delta, gl, programInfo, ctx2d);

    // Render upper menu
    renderFactoryMenu(delta, gl, programInfo, ctx2d);
};

let scoreboardFadeInter = 0;
let scoreboardFadeDuration = 0.5;

const renderScoreboard = (delta, gl, programInfo, ctx2d) => {
    // Render the block
    CUBE_MESH.setColor(COLOR_BLUE, gl, programInfo);
    let x = Board.boardCenter.x - Board.width / 2 - Board.GRID_PADDING - Board.FRAME_THICKNESS;
    let y = Board.boardCenter.y + Board.height / 2 + Board.GRID_PADDING + Board.FRAME_THICKNESS * 2;
    let w = Board.width + Board.FRAME_THICKNESS * 2 + Board.GRID_PADDING * 2;
    let h = VISIBLE_HEIGHT - y;
    CUBE_MESH.render(gl, x, y, 0, w, h, Board.BLOCK_WIDTH);

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
    ctx2d.fillStyle = "rgba(255, 255, 255," + (1 - cubicEaseIn(scoreboardFadeInter) * 0.5) + ")";

    // Get fonts
    let textHeight = 50;
    let monospaceFont = toBrowserY(72) + "px Digital-7";
    let cicleFont = toBrowserY(35) + "px New Cicle Fina";

    let blocksTextX = toBrowserX(x + w - Board.FRAME_THICKNESS);
    let textY = toBrowserH(y + h / 2 + textHeight / 2);

    ctx2d.textBaseline = "alphabetic";
    ctx2d.textAlign = "right";

    ctx2d.font = cicleFont;
    ctx2d.fillText("blocks", blocksTextX, textY);

    let blocksTextWidth = ctx2d.measureText(" blocks").width;

    let amountText = Math.floor(Data.currentBlocks);
    ctx2d.font = monospaceFont;
    ctx2d.fillText(amountText, blocksTextX - blocksTextWidth, textY);
};

let bpsButton;
let dialog;

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
const MENU_UPPER = 2;
const MENU_MAIN = 1;
const MENU_LOWER = 0;
let currentMenu = MENU_MAIN;

const openUpperMenu = (tab) => {
    yStart = globalYOffset;
    yEnd = VISIBLE_HEIGHT;
    yInter = 0;
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

    bpsButton = new Button(x, y, w, h, COLOR_ORANGE, "Factories", function () {
        openUpperMenu();
    });

};

const renderStatusBar = (delta, gl, programInfo, ctx2d) => {
    bpsButton.render(delta, gl, programInfo, ctx2d);
};