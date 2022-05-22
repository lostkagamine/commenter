import chalk from 'chalk';

const facades = {
    info: `${chalk.cyanBright('INFO')}   >`,
    debug: `${chalk.magenta('DEBUG')} >`,
    warn: `${chalk.yellowBright('WARN')}  >`,
    error: `${chalk.redBright('ERROR')} >`,
}

const actualLogFunc = (s, lvl) => console.log(`${facades[lvl]} ${s}`);

export default {
    info: s => actualLogFunc(s, 'info'),
    debug: s => actualLogFunc(s, 'debug'),
    warn: s => actualLogFunc(s, 'warn'),
    error: s => actualLogFunc(s, 'error'),
}