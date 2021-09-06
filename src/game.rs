use crate::button::Button;

struct Game {
    global_y_offset: f32,
    current_stage: Stage,
    current_upper_stage_menu: StageMenu,
    current_lower_stage_menu: StageMenu,

    board: Board,
    scoreboard: Scoreboard,

    y_start: f32,
    y_end: f32,
    y_inter: f32,

    factories_button: Button,
    stats_button: Button,
    settings_button: Button,
    upper_stage_back_button: Button,
    lower_stage_back_button: Button,
}

const Y_OFFSET_ANIMATE_DURATION: f32 = 0.5;

impl Default for Game {
    fn default() -> Self {}
}

impl Game {
    pub fn new() -> Self {
        let x = get_status_bar_x();
        let y = 0;
        let w = get_status_bar_width();
        let h = get_status_bar_height();

        let factories_button = Button::new(x, y, w, h / 2, COLOR_ORANGE, "Factories", || {
            self.open_upper_stage(StageMenu::FACTORIES);
        });
        let stats_button = Button::new(x, y + h / 2, w - h / 2, h / 2, COLOR_BLUE, "Stats", || {
            self.open_upper_stage(StageMenu::STATS);
        });
        let settings_button = Button::new(
            x + w - h / 2,
            y + h / 2,
            h / 2,
            h / 2,
            COLOR_GREEN,
            "settings",
            || {
                self.open_lower_stage(StageMenu::SETTINGS);
            },
        );
        settings_button.typeface = "Material Icons";
        settings_button.fontSize = 36;

        let upper_stage_back_button = Button::new(
            x,
            VISIBLE_HEIGHT - h / 2,
            w,
            h / 2,
            COLOR_RED,
            "keyboard_arrow_down",
            || {
                self.go_back_to_board();
            },
        );
        upper_stage_back_button.typeface = "Material Icons";
        upper_stage_back_button.fontSize = 36;

        let lower_stage_back_button =
            Button::new(x, 0, w, h / 2, COLOR_RED, "keyboard_arrow_up", || {
                self.go_back_to_board();
            });
        lower_stage_back_button.typeface = "Material Icons";
        lower_stage_back_button.fontSize = 36;

        Self {
            global_y_offset: 0,
            current_stage: Stage::MAIN,
            current_upper_stage_menu: StageMenu::FACTORIES,
            current_lower_stage_menu: StageMenu::SETTINGS,

            board: Board::new(),
            scoreboard: Scoreboard::new(),

            y_start: 0,
            y_end: 0,
            y_inter: 1,

            factories_button,
            settings_button,
            stats_button,
            upper_stage_back_button,
            lower_stage_back_button,
        }
    }

    fn logic(&self, delta: f32) {
        self.factoriesLogic(delta);
        self.board.logic(delta);
    }

    fn render_game(
        &self,
        delta: f32,
        gl: Gl,
        program_info: ProgramInfo,
        matrices: Matrices,
        ctx_2d: CanvasContext,
    ) {
        if self.yInter < 1 {
            self.yInter += delta / Y_OFFSET_ANIMATE_DURATION;
        }
        if (self.yInter > 1) {
            self.yInter = 1;
        }
        self.globalYOffset = self.yStart + quintEaseOut(self.yInter) * (self.yEnd - self.yStart);
        // Main stage
        if (self.yInter < 1 || currentStage == Stage.MAIN) {
            render_main_stage(delta, gl, program_info, ctx_2d, globalYOffset);
        }

        // Render upper stage
        if (self.yInter < 1 || currentStage == Stage.UPPER) {
            render_upper_stage(
                currentUpperStageMenu,
                delta,
                gl,
                program_info,
                ctx_2d,
                globalYOffset_ - VISIBLE_HEIGHT,
            );
        }

        // Render lower stage
        if (self.yInter < 1 || currentStage == Stage.LOWER) {
            render_lower_stage(
                currentLowerStageMenu,
                delta,
                gl,
                program_info,
                ctx_2d,
                globalYOffset_ + VISIBLE_HEIGHT,
            );
        }
    }

    fn render_main_stage(
        delta: f32,
        gl: Gl,
        program_info: ProgramInfo,
        ctx_2d: CanvasContext,
        y_offset: f32,
    ) {
        Board.render(gl, program_info, y_offset);
        render_scoreboard(delta, gl, program_info, ctx_2d, y_offset);
        render_status_bar(delta, gl, program_info, ctx_2d, y_offset);
    }

    fn render_upper_stage(
        stage_menu: StageMenu,
        delta: f32,
        gl: Gl,
        program_info: ProgramInfo,
        ctx_2d: CanvasContext,
        y_offset: f32,
    ) {
        ctx_2d.font = toBrowserH(DIALOG_TITLE_TEXT_HEIGHT) + "px New Cicle Fina";
        ctx_2d.fillStyle = "black";
        ctx_2d.textBaseline = "middle";
        ctx_2d.textBaseline = "center";
        ctx_2d.fillText(
            stage_menu,
            to_browser_x(get_status_bar_x() + get_status_bar_width() / 2),
            to_browser_y(get_status_bar_height() / 2 + y_offset),
        );
        match stage_menu {
            StageMenu::FACTORIES => renderFactoryMenu(delta, gl, programInfo, ctx_2d, y_offset),
            StageMenu::UPGRADES => (),
            StageMenu::STATS => renderStats(ctx_2d, y_offset),
            StageMenu::ACHIEVEMENTS => (),
            StageMenu::SETTINGS => todo!(),
        }
        upperStageBackButton.render(delta, gl, program_info, ctx_2d, y_offset);
    }

    fn render_lower_stage<Gl, CanvasContext>(
        stage_menu: StageMenu,
        delta: f32,
        gl: Gl,
        program_info: ProgramInfo,
        ctx_2d: CanvasContext,
        y_offset: f32,
    ) {
        // match (stageMenu) {
        //     StageMenu.SETTINGS:
        renderSettings(delta, gl, program_info, ctx_2d, y_offset);
        //       break;
        // }
        lowerStageBackButton.render(delta, gl, program_info, ctx_2d, y_offset);
    }

    fn open_upper_stage(menu: StageMenu) {
        currentStage = Stage.UPPER;

        currentUpperStageMenu = menu;
        self.yStart = globalYOffset;
        self.yEnd = VISIBLE_HEIGHT;
        self.yInter = 0;
    }

    fn open_lower_stage(menu: StageMenu) {
        currentStage = Stage.LOWER;

        currentLowerStageMenu = menu;
        self.yStart = globalYOffset;
        self.yEnd = -VISIBLE_HEIGHT;
        self.yInter = 0;
    }

    fn go_back_to_board() {
        currentStage = Stage.MAIN;

        self.yStart = globalYOffset;
        self.yEnd = 0;
        self.yInter = 0;
    }

    fn render_status_bar(
        delta: f32,
        gl: Gl,
        program_info: ProgramInfo,
        ctx_2d: CanvasContext,
        y_offset: f32,
    ) {
        factoriesButton.render(delta, gl, program_info, ctx_2d, y_offset);
        statsButton.render(delta, gl, program_info, ctx_2d, y_offset);
        settingsButton.render(delta, gl, program_info, ctx_2d, y_offset);
    }
}

struct ScoreboardStage {
    fade_inter: f32,
}

const SCOREBOARD_FADE_DURATION: f32 = 0.5;

impl Scoreboard {
    fn render(delta: f32, gl: Gl, program_info: ProgramInfo, ctx_2d: CanvasContext, y_offset: f32) {
        // Render the block
        cube_mesh.set_color(COLOR_BLUE, gl, program_info);
        let x = Board.boardCenter.x - Board.width / 2 - Board.GRID_PADDING - Board.FRAME_THICKNESS;
        let y =
            Board.boardCenter.y + Board.height / 2 + Board.GRID_PADDING + Board.FRAME_THICKNESS * 2;
        let w = Board.width + Board.FRAME_THICKNESS * 2 + Board.GRID_PADDING * 2;
        let h = VISIBLE_HEIGHT - y;
        cube_mesh.render(gl, x, y + y_offset, 0, w, h, Board.BLOCK_WIDTH);

        // set_the text color //
        // If there are falling blocks from the board
        if (Board.dump_blocks[0]) {
            if (scoreboardFadeInter < 1) {
                scoreboardFadeInter += delta / SCOREBOARD_FADE_DURATION;
                if (scoreboardFadeInter > 1) {
                    scoreboardFadeInter = 1
                };
            }
        } else {
            if (scoreboardFadeInter > 0) {
                scoreboardFadeInter -= delta / SCOREBOARD_FADE_DURATION;
                if (scoreboardFadeInter < 0) {
                    scoreboardFadeInter = 0
                };
            }
        }
        ctx_2d.fillStyle =
            "rgba(255, 255, 255,' + (1 - cubicEaseIn(scoreboardFadeInter) * 0.5) + ')";

        // get_fonts
        let text_height = 50;
        let monospace_font = to_browser_y(72) + "px Digital-7";
        let cicle_font = to_browser_y(35) + "px New Cicle Fina";

        let blocks_text_x = to_browser_x(x + w - Board.FRAME_THICKNESS);
        let text_y = toBrowserH(y + h / 2 + text_height / 2 + y_offset);

        ctx_2d.textBaseline = "alphabetic";
        ctx_2d.textAlign = "right";

        ctx_2d.font = cicle_font;
        ctx_2d.fillText("blocks", blocks_text_x, text_y);

        let blocks_text_width = ctx_2d.measure_text(" blocks").width;

        let amount_text = Math.floor(Data.current_blocks);
        ctx_2d.font = monospace_font;
        ctx_2d.fillText(amount_text, blocks_text_x - blocks_text_width, text_y);
    }
}

pub fn get_status_bar_x() {
    VISIBLE_WIDTH / 2 - get_status_bar_width() / 2
}

pub fn get_status_bar_width() {
    Board.width + Board.FRAME_THICKNESS * 2 + Board.GRID_PADDING * 2
}

pub fn get_status_bar_height() {
    Board.boardCenter.y - Board.height / 2 - Board.FRAME_THICKNESS * 3
}

enum Stage {
    Upper = 3,
    Main = 2,
    Lower = 1,
}

enum StageMenu {
    Factories,
    Upgrades,
    Stats,
    Achievements,
    Settings,
}

impl StageMenu {
    fn as_str(&self) -> &str {
        match self {
            StageMenu::Factories => "Factories",
            StageMenu::Upgrades => "Upgrades",
            StageMenu::Stats => "Statistics",
            StageMenu::Achievements => "Achievements",
            StageMenu::Settings => "Settings",
        }
    }
}
