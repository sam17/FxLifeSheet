// Telegram setup
const Telegraf = require("telegraf");

console.log("BOT TOKEN: ", process.env.TELEGRAM_BOT_TOKEN);
const bot = new Telegraf(process.env.TELEGRAM_BOT_TOKEN);

module.exports.bot = bot;
export {};
