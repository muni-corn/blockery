const COLOR_DISABLED_PURCHASE: i32 = 0x002caf;
const COLOR_DARK_GREEN: i32 = 0x009c3c;

const PRICE_INCREASE: f32 = 1.25; // 125%
const BASE_EMPTY_RATE: f32 = 100;

const PURCHASE_BUTTON_SIZE: f32 = Board.BLOCK_WIDTH * 3;
const STORAGE_BUTTON_HEIGHT: f32 = Board.BLOCK_WIDTH * 1.5;

const FACTORIES_PER_PAGE: u8 = 3;

struct Factory {
    name: String,
    base_price: f32,
    base_production_rate: f32,
    base_pollution_rate: f32,
    base_capacity: u128,

    emptying: bool,
    blocks_held: f64,
    amount_owned: u32,

    total_pollution_produced: f64,
    total_blocks_produced: u128,
}

impl Factory {
    fn new(name: &str, img_src: &str, base_price: f32, block_rate: f32, pollution_rate: f32, capacity: f64) -> Self {
        Self {
            name: name.to_string(),
            base_price,
            base_production_rate: block_rate,
            base_pollution_rate: pollution_rate,
            base_capacity: capacity,
            blocks_held: 0,
            amount_owned: 0,
            total_pollution_produced: 0,
            total_blocks_produced: 0,
            emptying: false,
        }
    }

    fn logic(&mut self, delta: f32, data: &mut Data) {
        self.produceBlocks(delta);
    }

    fn start_emptying(&mut self) {
        self.emptying = true;
    }

    fn empty_immediately(&mut self, data: &mut Data) {
        let emptied_blocks = self.blocksHeld.floor();
        data.current_blocks += emptied_blocks;
        self.blocksHeld -= emptied_blocks;
        self.emptying = false;
    }

    fn produce_blocks(&mut self, delta: f32, data: &mut Data) {
        if (self.emptying) {
            let emptied_blocks = self.totalEmptyRate * delta;
            if (emptied_blocks > self.blocksHeld) {
                // If we have emptied more blocks than self factory is holding,
                // set_delta to the amount of time put into emptying nonexistent
                // blocks so that it can be used after self if statement to produce
                // blocks
                delta = (emptied_blocks - Math.floor(self.blocksHeld)) / (self.totalEmptyRate);
                Data.current_blocks += Math.floor(self.blocksHeld);
                self.blocksHeld -= Math.floor(self.blocksHeld);
                self.emptying = false;
            } else {

                // Otherwise, just empty the blocks as expected and  from self
                // fn so that we do not produce blocks while we empty
                self.blocksHeld -= emptied_blocks;
                Data.current_blocks += emptied_blocks;

            }

        }


        if (!globalBlockProductionEnabled) { return; }


            // Produce blocks and pollution
            if (self.blocksHeld < self.totalCapacity) {

                let new_blocks = self.totalBlockRate * delta;
                self.totalBlocksProduced += new_blocks;
                self.blocksHeld += new_blocks;
                if (self.blocksHeld > self.totalCapacity) {
                    self.blocksHeld = self.totalCapacity;
                }

                let new_pollution = self.totalPollutionRate * delta;
                data.current_pollution += new_pollution;
                self.total_pollution_produced += new_pollution;
            }
    }

    fn is_affordable(&self, balance: u128) -> bool {
        balance >= self.price
    }

    fn is_base_price_affordable(&self, balance: u128) -> bool {
        balance >= self.basePrice
    }

    fn buy(data: &mut Data) {
        if (self.isAffordable(data.current_blocks)) {
            data.current_blocks -= self.price;
            self.amountOwned += 1;
            Firework::new(self.imageButton.x + self.imageButton.w / 2, self.imageButton.y + self.imageButton.h / 2);
        }
    }

    fn get_total_block_rate() {
        self.amountOwned * self.singularBlockRate
    }

    fn get_singular_block_rate() {
        self.baseProductionRate * self.localBlockRateMultiplier * globalBlockRateMultiplier
    }

    fn get_total_capacity() {
        self.amountOwned * self.baseCapacity * self.localCapactiyMultiplier * globalCapacityMultiplier
    }

    fn get_total_pollution_rate() {
        self.amountOwned * self.basePollutionRate * self.localPollutionMultiplier * globalPollutionMultiplier
    }

    fn get_total_empty_rate() {
        self.amountOwned * BASE_EMPTY_RATE * globalEmptyRateMultiplier
    }

    fn get_price() {
        Math.floor(self.basePrice * Math.pow(PRICE_INCREASE, self.amountOwned) * self.localPriceMultiplier * globalPriceMultiplier)
    }

    /** Includes padding at the bottom. */
    fn get_info_card_height() {
        PURCHASE_BUTTON_SIZE + STORAGE_BUTTON_HEIGHT + UI_PADDING * 2
    }

    fn get_visible_on_page() {
        Math.floor(self.index / FACTORIES_PER_PAGE) == currentFactoryPage
    }

    fn render_options<Gl, CanvasContext>(delta: f23, gl: Gl, program_info: programInfo, ctx_2d: CanvasContext, y_offset: f32) {
        if (!self.visibleOnPage || self.index > factories_unlocked) {
            self.imageButton.enabled = false;
            self.progressButton.enabled = false;

        }

        let hidden = factories_unlocked == self.index;
        if (!hidden && self.wasHidden) {
            self.imageButton.img_src = self.img_src;
            self.wasHidden = hidden;
        }

        // Assign an easier variable for UI_PADDING
        let p = UI_PADDING;

        let status_bar_height = getStatusBarHeight();
        let y = status_bar_height + (get_page_changer_button_y() - status_bar_height) / 2 - (Factory.infoCardHeight * FACTORIES_PER_PAGE - UI_PADDING * 2) / 2 + self.index % FACTORIES_PER_PAGE * Factory.infoCardHeight + y_offset;

        self.imageButton.enabled = Data.current_blocks >= self.price && !hidden;
        self.imageButton.y = y;
        self.imageButton.text = if (self.amountOwned > 0) { self.amountOwned } else { 0 };
        self.imageButton.render(delta, gl, program_info, ctx_2d);

        ctx_2d.textAlign = "left";
        ctx_2d.textBaseline = "top";
        ctx_2d.fillStyle = "black";

        // Header
        let text_x = to_browser_x(self.imageButton.x + self.imageButton.w + UI_PADDING);
        let text_y = to_browser_y(self.imageButton.y);
        ctx_2d.font = toBrowserH(DIALOG_TITLE_TEXT_HEIGHT) + "px New Cicle Fina";
        ctx_2d.fillText(if hidden { "Under Construction" } else { self.name }, text_x, text_y);

        // Info //
        ctx_2d.font = getSansFont();
        ctx_2d.textBaseline = "alphabetical";
        text_y = to_browser_y(self.imageButton.y + DIALOG_TITLE_TEXT_HEIGHT * 1.15);

        ctx_2d.fillText("Costs ' + self.price.to_locale_string() + ' for +' + self.singularBlockRate.to_locale_string() + ' bps", text_x, text_y);

        // If this factory is owned...
        if (self.amountOwned > 0) {
            // ...declare "full" if its capacity has been reached...
            if (self.blocksHeld == self.totalCapacity) {
                ctx_2d.fillText("Full", text_x, text_y + toBrowserH(UI_SANS_TEXT_HEIGHT) * 1.15);
                // ...or, display how much time it will take until it is full (if not emptying)
                // or empty (if emptying)
            } else {
                // get_the time remaining...
                let time_left = 
                    if (self.emptying) {
                        // ...until empty
                        time_left = self.blocksHeld / self.totalEmptyRate;
                    } else {
                        // ... until full
                        time_left = (self.totalCapacity - self.blocksHeld) / self.totalBlockRate;
                    };

                // By default, time is measured in seconds but reduced to larger
                // time units if timeLeft is too large
                let time_unit = "seconds";

                if (time_left >= 3600 * 24 * 7) {
                    time_left /= 3600 * 24 * 7;
                    time_unit = "weeks";
                } else if (time_left >= 3600 * 24) {
                    time_left /= 3600 * 24;
                    time_unit = "days";
                } else if (time_left >= 3600) {
                    time_left /= 3600;
                    time_unit = "hours";
                } else if (time_left >= 60) {
                    time_left /= 60;
                    time_unit = "minutes";
                }

                if (time_unit == "seconds") {
                    // round (up) to tenths of seconds
                    time_left = ((time_left * 10).ceil() / 10).toFixed(1);
                } else {
                    // or just ceiling everything else
                    time_left = (time_left).ceil();
                }

                format!("");
                    ctx_2d.fillText((if self.emptying { "Empty in " } else { "Full in " }) + timeLeft + " " + time_unit, text_x, text_y + toBrowserH(UI_SANS_TEXT_HEIGHT) * 1.15);
            }

            self.progressButton.enabled = !self.emptying;
            self.progressButton.progress = self.blocksHeld / self.totalCapacity;
            self.progressButton.text = Math.floor(self.blocksHeld) + " / " + self.totalCapacity;
            self.progressButton.y = y + self.imageButton.h;
            self.progressButton.render(delta, gl, program_info, ctx_2d);
        }
    }
}

// Excuse this mess
const FACTORIES: [Factory; 10] = [
    Factory::new("Blocksmith', 'img/smit.png", 500, 0.5, 0.1, 50),
    Factory::new("Cottage Factory', 'img/cott.png", 500 * 15, 0.5 * 4, 0.1 * 5, 50 * 6),
    Factory::new("Block Mine', 'img/mine.png", 500 * 15 * 15, 0.5 * 4 * 8, 0.1 * 5 * 10, 50 * 6 * 12),
    Factory::new("Powerhouse', 'img/powh.png", 500 * 15 * 15 * 15, 0.5 * 4 * 8 * 12, 0.1 * 5 * 10 * 15, 50 * 6 * 12 * 18),
    Factory::new("Cloudmaker', 'img/clmk.png", 500 * 15 * 15 * 15 * 15, 0.5 * 4 * 8 * 12 * 16, 0.1 * 5 * 10 * 15 * 20, 50 * 6 * 12 * 18 * 24),
    Factory::new("Block Volcano', 'img/volc.png", 500 * 15 * 15 * 15 * 15 * 15, 0.5 * 4 * 8 * 12 * 16 * 20, 0.1 * 5 * 10 * 15 * 20 * 25, 50 * 6 * 12 * 18 * 24 * 30),
    Factory::new("Moon Block Farm', 'img/mnfm.png", 500 * 15 * 15 * 15 * 15 * 15 * 15, 0.5 * 4 * 8 * 12 * 16 * 20 * 24, 0.1 * 5 * 10 * 15 * 20 * 25 * 30, 50 * 6 * 12 * 18 * 24 * 30 * 36),
    Factory::new("Planetary Block Storm', 'img/plsm.png", 500 * 15 * 15 * 15 * 15 * 15 * 15 * 15, 0.5 * 4 * 8 * 12 * 16 * 20 * 24 * 28, 0.1 * 5 * 10 * 15 * 20 * 25 * 30 * 35, 50 * 6 * 12 * 18 * 24 * 30 * 36 * 42),
    Factory::new("Star Reactor', 'img/star.png", 500 * 15 * 15 * 15 * 15 * 15 * 15 * 15 * 15, 0.5 * 4 * 8 * 12 * 16 * 20 * 24 * 28 * 32, 0.1 * 5 * 10 * 15 * 20 * 25 * 30 * 35 * 40, 50 * 6 * 12 * 18 * 24 * 30 * 36 * 42 * 48),
    Factory::new("Interdimensional Gateway', 'img/dmgt.png", 500 * 15 * 15 * 15 * 15 * 15 * 15 * 15 * 15 * 15, 0.5 * 4 * 8 * 12 * 16 * 20 * 24 * 28 * 32 * 36, 0.1 * 5 * 10 * 15 * 20 * 25 * 30 * 35 * 40 * 45, 50 * 6 * 12 * 18 * 24 * 30 * 36 * 42 * 48 * 54)
    // The Everything Dimension?
];

fn factories_logic(delta: f32) {
    for f in FACTORIES {
        factories[prop].logic(delta);
    }
}

const PAGE_CHANGER_BUTTON_WIDTH: i32 = 150;
const PAGE_CHANGER_BUTTON_HEIGHT: i32 = 50;

fn get_page_changer_button_y() {
    VISIBLE_HEIGHT - getStatusBarHeight() * 1.5 - UI_PADDING - PAGE_CHANGER_BUTTON_HEIGHT
}

fn get_max_page() {
    Math.floor(factories_unlocked / FACTORIES_PER_PAGE)
}

fn render_factory_menu<Gl, CanvasContext>(delta: f32, gl: Gl, program_info: ProgramInfo, ctx_2d: CanvasContext, y_offset: f32) {
    for f in FACTORIES {
        f.renderOptions(delta, gl, program_info, ctx_2d, y_offset);
    }
    if (previousPageButton) {
        previousPageButton.render(delta, gl, program_info, ctx_2d, y_offset);
    }

    if (nextPageButton) {
        nextPageButton.render(delta, gl, program_info, ctx_2d, y_offset);
    }

    ctx_2d.font = toBrowserH(UI_SANS_TEXT_HEIGHT * 1.5) + "px New Cicle Fina";
    ctx_2d.fillStyle = "black";
    ctx_2d.textBaseline = "middle";
    ctx_2d.textAlign = "center";
    ctx_2d.fillText((currentFactoryPage + 1) + " / " + (get_max_page() + 1), to_browser_x(VISIBLE_WIDTH / 2), to_browser_y(get_page_changer_button_y() + nextPageButton.h / 2 + y_offset));

    render_factory_menu_scoreboard(gl, program_info, ctx_2d, y_offset);
}

fn render_factory_menu_scoreboard<Gl, CanvasContext>(gl: Gl, program_info: ProgramInfo, ctx_2d: CanvasContext, y_offset: f32) {
    // Render the block
    cube_mesh.set_color(COLOR_BLUE, gl, program_info);
    let h = getStatusBarHeight();
    let w = getStatusBarWidth();
    let x = getStatusBarX();
    let y = VISIBLE_HEIGHT - h * 1.5 + y_offset;
    cube_mesh.render(gl, x, y, 0, w, h, Board.BLOCK_WIDTH);

    // set_the text color
    ctx_2d.fillStyle = "white";

    // get_fonts
    let text_height = 50;
    let monospace_font = toBrowserH(text_height) + "px Digital-7";
    let cicle_font = toBrowserH(text_height / 2) + "px New Cicle Fina";

    let blocks_text_x = to_browser_x(x + w - Board.FRAME_THICKNESS);
    let text_y = to_browser_y(y + h / 2 + text_height / 2);

    ctx_2d.font = cicle_font;
    let right_indent = Math.max(ctx_2d.measure_text(" blocks').width, ctx_2d.measure_text(' stored").width);
    ctx_2d.textBaseline = "center";

    ctx_2d.textAlign = "left";
    ctx_2d.fillText(" blocks", blocks_text_x - right_indent, to_browser_y(y + h / 3));
    ctx_2d.fillStyle = "rgba(255, 255, 255, 0.75)";
    ctx_2d.fillText(" stored", blocks_text_x - right_indent, to_browser_y(y + h * 2 / 3));

    ctx_2d.textAlign = "right";
    let amount_text = Math.floor(Data.current_blocks);
    let total_stored_blocks = 0;
    for f in FACTORIES {
        total_stored_blocks += FACTORIES[prop].blocksHeld;
    }
    ctx_2d.font = monospace_font;
    ctx_2d.fillText("+" + Math.floor(total_stored_blocks), blocks_text_x - right_indent, to_browser_y(y + (h * 2 / 3)));
    ctx_2d.fillStyle = "white";
    ctx_2d.fillText(amount_text, blocks_text_x - right_indent, to_browser_y(y + (h / 3)));
}

fn check_page_buttons() {
    if (!nextPageButton) {
        nextPageButton = Button(getStatusBarX() + getStatusBarWidth() - PAGE_CHANGER_BUTTON_HEIGHT, getPageChangerButtonY::new(), PAGE_CHANGER_BUTTON_HEIGHT, PAGE_CHANGER_BUTTON_HEIGHT, COLOR_ORANGE, "keyboard_arrow_right", || {
            if (currentFactoryPage < get_max_page()) {
                currentFactoryPage += 1;
            }
            check_page_buttons();
        });
        nextPageButton.typeface = "Material Icons";
        nextPageButton.fontSize = 36;
    }

    if (!previousPageButton) {
        previousPageButton = Button(getStatusBarX(), getPageChangerButtonY::new(), PAGE_CHANGER_BUTTON_HEIGHT, PAGE_CHANGER_BUTTON_HEIGHT, COLOR_ORANGE, "keyboard_arrow_left", || {
            if (currentFactoryPage > 0) {
                currentFactoryPage -= 1;
            }
            check_page_buttons();
        });
        previousPageButton.typeface = "Material Icons";
        previousPageButton.fontSize = 36;
    }

    nextPageButton.enabled = currentFactoryPage < get_max_page();
    previousPageButton.enabled = currentFactoryPage > 0;
}

// Check to see if new pages have become available as blocks are collected
// Listeners.blockCountListeners.push({
//     onBlockCount: |blocks| {
//         let numFactories = Object.keys(factories).length;
//         if (factories_unlocked < numFactories) {
//             let factory;
//             let factoryIsAffordable;
//             do {
//                 factory = factories[Object.keys(factories)[factories_unlocked]];
//                 if (!factory)

//                     factoryIsAffordable = factory.isBasePriceAffordable(blocks);
//                 if (factoryIsAffordable)
//                     factories_unlocked++;
//             } while (factoryIsAffordable && numFactories);
//         }
//         checkPageButtons();
//     }
// });
