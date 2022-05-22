const DEFAULT_GRAY: f32 = 0.95;
const BLOCK_WIDTH: f32 = 30;
const BLOCK_GAP: i32 = 15;
const FRAME_THICKNESS: f32 = BLOCK_WIDTH / 2.0;
const ROWS: i32 = 14;
const COLUMNS: i32 = 10;

struct Board {
    blink_time: f32,
    blink_end: f32,
    blink_rgb: [u8; 3],
    light_color: f32,

    board_center: [f32; 2],
    y_offset: f32,

    fill_rate: f32,
    queue: [Option<Block>; COLUMNS],
    block_grid: [[Option<Block>; ROWS]; COLUMNS],

    pending_fill_time: f32,

    dump_blocks: Vec<Block>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            blink_time: 0.0,
            blink_end: 0.0,
            blink_rgb: [0, 0, 0],
            light_color: 0.0,

            board_center: [0, 0],

            y_offset: 0.0,

            fill_rate: 2 * 100, // The rate at which cubes fill in units per second (100 units per
            // block)
            queue: [None; COLUMNS],
            block_grid: [[None; ROWS]; COLUMNS],

            pending_fill_time: 0,

            dump_blocks: Vec::new(),
        }
    }
}

impl Board {
    /// TODO: old code.
    fn drop_queue(&self) {
        let mut board_full = true;
        for i in 0..self.COLUMNS {
            if self.board[0][i].is_none() {
                self.board[0][i] = self.queue[i];
                self.queue[i] = None;
                board_full = false;
            }
        }
        if board_full {
            self.pending_fill_time = 0
        };
    }

    /// TODO: old code.
    fn fill_queue(&self, delta: f32) {
        if !global_block_production_enabled {
            return;
        }

        self.pending_fill_time += delta;
        let should_drop = true;
        for i in 0..COLUMNS {
            if !self.queue[i] {
                self.queue[i] = Block::new();
            }
            let block = self.queue[i];

            if (i == 0 || self.queue[i - 1].is_full()) && !block.is_full() {
                let f = self.queue[i].fill;
                self.queue[i].fill += self.fill_rate * self.pending_fill_time;
                self.pending_fill_time -= (self.queue[i].fill - f) / self.fill_rate;
                should_drop = false;
            }
        }
        if should_drop {
            self.drop_queue();
        }
    }

    /// TODO: old code.
    fn get_grid_code(&self) {
        let code = "";
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                if !self.board[r][c] {
                    code += "0";
                } else {
                    match self.board[r][c].color {
                        COLOR_RED => code += "r",
                        COLOR_ORANGE => code += "o",
                        COLOR_GREEN => code += "g",
                        COLOR_BLUE => code += "b",
                    }
                }
            }
        }
        code
    }

    /// TODO: old code.
    fn init<CanvasContext>(
        ctx_2d: CanvasContext,
        board_code: &str,
        mouse_listeners: &mut Listeners,
    ) -> Self {
        let grid = [[None; ROWS]; COLUMNS];
        let queue = [None; COLUMNS];
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                let code_index = r * COLUMNS + c;
                let color;
                if board_code && (color = board_code[code_index]) != "0" {
                    grid[r][c] = Some(Block::new(color));
                    continue;
                }
                grid[r][c] = None;
            }
        }

        for i in 0..COLUMNS {
            queue[i] = None;
        }

        mouse_listeners.push(self);

        todo!()
    }

    /// TODO: old code.
    fn logic(delta: f32) {
        let is_blink_color = blinkR != 0 || blinkG != 0 || blinkB != 0;
        if (is_blink_color && (blinkTime < blinkEnd || blinkEnd == 0)) {
            blinkTime += delta * 10;
        } else {
            blinkTime = blinkEnd;
        }

        let foo = Math.abs(Math.sin(blinkTime)) * 3 / 4;
        let r = foo * blinkR + (1 - foo) * defaultGray;
        let g = foo * blinkG + (1 - foo) * defaultGray;
        let b = foo * blinkB + (1 - foo) * defaultGray;
        lightColor = rgbToInt(r, g, b);

        // Do board logic
        // Start from the bottom row and move up
        for r in (self.ROWS - 1)..=0 {
            for c in 0..COLUMNS {
                let block = self.board[r][c];
                if (block != null) {
                    // If a block here exists, calculate physics on
                    // self block
                    block.blockLogic(delta, r, c);

                    // If a poison block reaches the bottom, remove it
                    // (and inflict punishment >:))
                    if (r == self.ROWS - 1 && block.color == COLOR_TOXIC) {
                        self.pushBlockToDump(r, c);
                    }
                } else if (r > 0) {
                    // If this slot doesn't have a block and this
                    // isn't the first row, then move the block
                    // above this slot to this slot
                    for i in r..=0 {
                        let block_above = self.board[i][c];
                        if (block_above != null) {
                            self.board[r][c] = block_above;
                            self.board[i][c] = null;
                            break;
                        }
                    }
                }
            }
        }

        // Queue logic
        self.fillQueue(delta);

        for i in 0..COLUMNS {
            if (self.queue[i]) {
                self.queue[i].blockLogic(delta, -1, i);
            }
        }

        // Do trash block logic
        for item in self.dump_blocks {
            item.blockLogic(delta, item.row, item.col);
            if (item.y >= item.destY && !item.falling) {
                // (For poison blocks)
                // Make the block fall if it is at or past it's
                // destination y position
                item.fall();
            }
            if (item.gone) {
                self.countBlock(self.dump_blocks.splice(index, 1)[0]);
            }
        }
    }

    /// TODO: old code.
    fn render<Gl>(&self, gl: Gl, program_info: ProgramInfo, y_offset: f32) {
        self.y_offset_ = y_offset;
        self.renderBoardFrame(gl, program_info, y_offset);
        self.renderGrid(gl, program_info, y_offset);
        self.renderQueue(gl, program_info, y_offset);
        self.renderDumpBlocks(gl, program_info, y_offset);
    }

    /// TODO: old code.
    fn render_queue<Gl>(gl: Gl, program_info: ProgramInfo, y_offset: f32) {
        // Render the queue
        for i in 0..COLUMNS {
            let block = self.queue[i];
            if (block.is_some()) {
                block.renderBlock(gl, program_info, y_offset);
            }
        }
    }

    /// TODO: old code.
    fn render_grid<Gl>(gl: Gl, program_info: ProgramInfo, y_offset: f32) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                let block = self.board[r][c];
                if (block.is_some()) {
                    block.renderBlock(gl, program_info, y_offset);
                }
            }
        }
    }

    /// TODO: old code.
    fn render_dump_blocks<Gl>(gl: Gl, program_info: ProgramInfo, y_offset: f32) {
        // Render any falling blocks
        for item in dump_blocks {
            item.renderBlock(gl, program_info, y_offset);
        }
    }

    /// TODO: old code.
    fn render_board_frame<Gl>(gl: Gl, program_info: ProgramInfo, y_offset: f32) {
        // Declare a shorter name for frame thickness
        let t = self.FRAME_THICKNESS;

        cube_mesh.set_color(0xffffff, gl, program_info);

        // Left side of frame
        let lw = t;
        let lh = Board.height + self.GRID_PADDING;
        let lx = self.boardCenter.x - self.width / 2 - self.GRID_PADDING - t;
        let ly = self.boardCenter.y - self.height / 2 + y_offset;
        cube_mesh.render(gl, lx, ly, 0, lw, lh, t);

        // Right side of frame
        let rw = lw;
        let rh = lh;
        let rx = self.boardCenter.x + self.width / 2 + self.GRID_PADDING;
        let ry = ly;
        cube_mesh.render(gl, rx, ry, 0, rw, rh, t);

        //  Bottom of the frame
        let bh = t;
        let bw = Board.width + t * 2 + self.GRID_PADDING * 2;
        let bx = lx;
        let by = self.boardCenter.y + self.height / 2 + self.GRID_PADDING + y_offset;
        cube_mesh.render(gl, bx, by, 0, bw, bh, t);

        // Lights
        cube_mesh.set_color(lightColor, gl, program_info);
        cube_mesh.render(gl, lx, ly - t, 0, t, t, t);
        cube_mesh.render(gl, rx, ry - t, 0, t, t, t);
    }

    /// TODO: old code.
    fn blink_lights(color: i32, count: u8) {
        color = intToRGB(color);

        blinkR = color.r;
        blinkG = color.g;
        blinkB = color.b;

        blinkEnd = Math.PI * count;
        blinkTime = 0;
    }

    /// Transfers the block at the given row and
    /// column to the array of falling (dump) blocks.
    ///
    /// TODO: old code.
    fn push_block_to_dump(row: i32, col: i32) {
        self.dump_blocks.push(self.board[row][col]);
        self.board[row][col] = null;
    }

    /// TODO: old code.
    fn count_block(block: &Block, data: &mut Data) {
        if (block.color == COLOR_TOXIC) {
            data.lifetime_blocks_by_color.toxic += 1;
        }

        data.current_blocks += 1;
        match (block.color) {
            COLOR_RED => data.lifetime_blocks_by_color.red += 1,
            COLOR_ORANGE => data.lifetime_blocks_by_color.orange += 1,
            COLOR_GREEN => data.lifetime_blocks_by_color.green += 1,
            COLOR_BLUE => data.lifetime_blocks_by_color.blue += 1,
        }
    }

    /// TODO: old code.
    fn to_grid_x(col: i32) {
        self.boardCenter.x - self.width / 2 + (col * (self.BLOCK_WIDTH + self.SPACING))
    }

    /// TODO: old code.
    fn to_grid_y(row: i32) {
        let top = self.boardCenter.y - self.height / 2;
        if (row == -1) {
            top
        } else {
            top +
                // Add a row and spacing to accomodate the queue
                (row + 1) * (self.BLOCK_WIDTH + self.SPACING) +
                self.SPACING;
        }
    }

    /// TODO: old code.
    fn get_width() {
        self.COLUMNS * (self.BLOCK_WIDTH + self.SPACING) - self.SPACING
    }

    /// TODO: old code.
    fn get_height() {
        // Here we include the block queue at the top, which has
        // double the spacing below it.
        (self.ROWS + 1) * (self.BLOCK_WIDTH + self.SPACING)
    }

    /// Kicks a block and any matching neighbors out
    /// of the board grid if said matching neighbor(s)
    /// exist. s true if so, and false otherwise.
    ///
    /// TODO: old code.
    fn remove_if_matching_neighbors(&self, row: i32, col: i32) -> bool {
        let block = self.board[row][col];

        if (!block) {
            return false;
        }

        let had_match = false;

        if (row > 0 && self.match_with_neighbor(row, col, row - 1, col)) {
            had_match = true;
        }
        if (row < self.ROWS - 1 && self.match_with_neighbor(row, col, row + 1, col)) {
            had_match = true;
        }

        if (col > 0 && self.match_with_neighbor(row, col, row, col - 1)) {
            had_match = true;
        }
        if (col < self.COLUMNS - 1 && self.match_with_neighbor(row, col, row, col + 1)) {
            had_match = true;
        }

        if (had_match) {
            self.push_block_to_dump(row, col);
            true
        } else {
            false
        }
    }

    /// Matches two blocks together given their rows and columns. If the blocks match, the
    /// neighboring blocks will be checked for more matching neighbors, and the fn will
    /// true.
    ///
    /// TODO: old code.
    fn match_with_neighbor(
        &self,
        block_to_remove_row: i32,
        block_to_remove_column: i32,
        neighbor_row: i32,
        neighbor_column: i32,
    ) -> bool {
        let block_to_remove = self.board[block_to_remove_row][block_to_remove_column];
        let neighbor = self.board[neighbor_row][neighbor_column];

        if (!block_to_remove || !neighbor) {
            return false;
        }

        if (neighbor.color == block_to_remove.color) {
            block_to_remove.fall();
            // If self block has already been kicked off the board, don't bother checking
            // it for more matching neighbors
            if (!neighbor.falling) {
                self.remove_if_matching_neighbors(neighbor_row, neighbor_column);
            }
            true
        } else {
            false
        }
    }

    /// TODO: old code.
    fn on_click(&self, mx: i32, my: i32, data: &mut Data) {
        let block_cell_size = BLOCK_WIDTH + BLOCK_GAP;
        let row = ((my - self.board_center.y - self.y_offset_ + (self.height / 2)
            - self.BLOCK_GAP)
            / block_cell_size
            - 1)
        .floor();
        let col = ((mx - self.board_center.x + (self.width / 2)) / block_cell_size).floor();

        if (row >= 0 && row < self.ROWS && col >= 0 && col < self.COLUMNS) {
            if (self.remove_if_matching_neighbors(row, col)) {
                data.lifetime_clicks.successful += 1;
            } else {
                data.lifetime_clicks.failed += 1;
            }
        }
    }
}
