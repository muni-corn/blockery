#[derive(Default)]
pub struct Data {
    current_blocks: u128,
    current_pollution: f64,
    lifetime_blocks_by_color: ColorStats,
    lifetime_clicks: ClickStats,
    board_code: String,
    factories_unlocked: i32,
}

struct ColorStats {
    red: u128,
    orange: u128,
    green: u128,
    blue: u128,
    toxic: u128,
    golden: u128
}

impl Default for ColorStats {
    fn default() -> Self {
        Self {
            red: 0,
            orange: 0,
            green: 0,
            blue: 0,
            toxic: 0,
            golden: 0
        }
    }
}

struct ClickStats {
    failed: u64,
    successful: u64,
}

impl Default for ClickStats {
    fn default() -> Self {
        Self {
            failed: 0,
            successful: 0,
        }
    }
}

const KEY_DATABASE_VERSION: &str = "version";
const KEY_BLOCKERY_VERSION: &str = "blockeryversion";
const KEY_GRID: &str = "grid";
const KEY_CURRENT_BLOCKS: &str = "current_blocks";
const KEY_CURRENT_POLLUTION: &str = "current_pollution";
const KEY_LIFETIME_RED: &str = "red";
const KEY_LIFETIME_ORANGE: &str = "orange";
const KEY_LIFETIME_GREEN: &str = "green";
const KEY_LIFETIME_BLUE: &str = "blue";
const KEY_SUCCESSFUL_CLICKS: &str = "successfulClicks";
const KEY_FAILED_CLICKS: &str = "failedClicks";
const KEY_FACTORIES_UNLOCKED: &str = "factories_unlocked";

const DATABASE_VERSION: i32 = 1;
const BLOCKERY_VERSION: &str = "0.0.0-alpha2";

impl Data {
    fn load<CanvasContext>(ctx_2d: CanvasContext) {
        let db_version = Number.parseInt(local_storage.getItem(KEY_DATABASE_VERSION)) || 0;
        if !db_version {
            console.log("No database found. No data will be loaded.");
            Dialog::new(ctx_2d, "Welcome to Blockery!", "Glad you're here! Blocks can be collected by selecting those that are the same color as the blocks they are touching. Once you save up enough blocks, you can purchase factories that will make more blocks for you. Each factory can only store a certain number of blocks and will need to be emptied once in a while. A factory that is storing too many blocks will stop producing any more! As you collect more and more blocks, different and various types of factories will be available. There are lots to be discovered here.").setButton(BUTTON_POSITIVE, "Let's go!", |dialog| {
                dialog.dismiss();
            }).show();

            self.current_blocks = 0;
        }

        // Alert the user of new updates :)
        let blockery_version = local_storage.getItem(KEY_BLOCKERY_VERSION);
        if (BLOCKERY_VERSION != blockery_version) {
            loadTextResource("releaseLog.txt", |error, result| {
                if (error)
                { addError(error); }
                else {
                    Dialog::new(ctx_2d, "Welcome to Blockery " + BLOCKERY_VERSION + "!", "Here's what's new: \n" + result).show();
                }
            });
        }

        // Loading procedure for version 1 //
        self.current_blocks = Number.parseInt(local_storage.getItem(KEY_CURRENT_BLOCKS)) || 0;

        self.board_code = local_storage.getItem(KEY_GRID);

        self.lifetime_blocks_by_color.red = Number.parseInt(local_storage.getItem(KEY_LIFETIME_RED)) || 0;
        self.lifetime_blocks_by_color.orange = Number.parseInt(local_storage.getItem(KEY_LIFETIME_ORANGE)) || 0;
        self.lifetime_blocks_by_color.green = Number.parseInt(local_storage.getItem(KEY_LIFETIME_GREEN)) || 0;
        self.lifetime_blocks_by_color.blue = Number.parseInt(local_storage.getItem(KEY_LIFETIME_BLUE)) || 0;

        self.lifetime_clicks.successful = Number.parseInt(local_storage.getItem(KEY_SUCCESSFUL_CLICKS)) || 0;
        self.lifetime_clicks.failed = Number.parseInt(local_storage.getItem(KEY_FAILED_CLICKS)) || 0;

        // TODO: Load factories
        /*
           factories_unlocked = Number.parseInt(local_storage.getItem(KEY_FACTORIES_UNLOCKED)) || 0;
           for (let prop in factories) {
           let val = local_storage.getItem(prop);
           if (!val)
           continue;
           factories[prop].applyCode(db_version, val);
           }
           */
    }

    fn save() {
        local_storage.set_item(KEY_DATABASE_VERSION, DATABASE_VERSION);
        local_storage.set_item(KEY_BLOCKERY_VERSION, BLOCKERY_VERSION);

        // Saving procedure for version 1 //
        local_storage.set_item(KEY_CURRENT_BLOCKS, self.current_blocks);
        local_storage.set_item(KEY_CURRENT_POLLUTION, self.current_pollution);

        // Save the board
        local_storage.set_item(KEY_GRID, Board.get_grid_code());

        // Save user stats
        local_storage.set_item(KEY_LIFETIME_RED, self.lifetime_blocks_by_color.red);
        local_storage.set_item(KEY_LIFETIME_ORANGE, self.lifetime_blocks_by_color.orange);
        local_storage.set_item(KEY_LIFETIME_GREEN, self.lifetime_blocks_by_color.green);
        local_storage.set_item(KEY_LIFETIME_BLUE, self.lifetime_blocks_by_color.blue);
        local_storage.set_item(KEY_SUCCESSFUL_CLICKS, self.lifetime_clicks.successful);
        local_storage.set_item(KEY_FAILED_CLICKS, self.lifetime_clicks.failed);

        // TODO: Save factories
        /*
           local_storage.set_item(KEY_FACTORIES_UNLOCKED, factories_unlocked);
           for (let prop in factories) {
           local_storage.set_item(prop, factories[prop].save_code);
           }
           */

        Board.blinkLights(COLOR_GREEN, 1);
        sendNotification("Game saved!", 1.5);
    }

    fn reset() {
        saveOnBeforeUnload = false;
        Dialog::new(window.ctx_2d, "Reset everything?", "This action can't be undone! You will lose all of your blocks, factories, and everything you've put so much work into :(")
            .setButton(BUTTON_POSITIVE, "Sure", |dialog| {
                dialog.dismiss();
                RedDialog::new(window.ctx_2d, "Final warning!", "Do you really want to reset everything? All of your blocks, factories, and data will be erased. Gone forever. You will not be able to recover them.")
                    .setButton(BUTTON_POSITIVE, "Go ahead", |dialog| {
                        local_storage.clear();
                        window.location.reload(false);
                    })
                .setButton(BUTTON_NEGATIVE, "Never mind", |dialog| {
                    dialog.dismiss();
                })
                .show();
                })
        .setButton(BUTTON_NEGATIVE, "Nope", |dialog| {
            dialog.dismiss();
        })
        .show();
    }

   fn set_current_blocks(&mut self, val: u128) {
      self.current_blocks = val;
      Listeners.invokeBlockCountListeners(val);
   }

   fn get_current_blocks(&self) -> u128 {
       self.current_blocks
   }

   fn get_lifetime_pollution(&self) {
      let total = 0;
      for f in FACTORIES {
          total += f.total_pollution_produced;
      }
       total
   } 
}
