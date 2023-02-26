"use strict";
exports.__esModule = true;
var Telegraf = require("telegraf");
console.log("BOT TOKEN: ", process.env.TELEGRAM_BOT_TOKEN);
var bot = new Telegraf(process.env.TELEGRAM_BOT_TOKEN);
module.exports.bot = bot;
//# sourceMappingURL=telegram.js.map