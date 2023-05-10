const fs = require('fs');
const path = require('path');
const ps = require('child_process');
const readline = require('readline');

const CURRENT_WORKSPACE = path.join(__dirname, './');
const LC_MOD_FILE_ROOT = `${CURRENT_WORKSPACE}/src/leetcode/mod.rs`;
const PROJECT_MAIN = `${CURRENT_WORKSPACE}/src/main.rs`; 


const run = () => {
    const modname = process.argv.slice(2)[0];

    if (!modname || fs.existsSync(`${CURRENT_WORKSPACE}/src/leetcode/${modname}`)) {
        console.log('\ndirectory already exist.')
        return;
    }

    console.log(CURRENT_WORKSPACE, modname);
    ps.execSync(`mkdir ${CURRENT_WORKSPACE}/src/leetcode/${modname}`)

    const lc_mod_path = `${CURRENT_WORKSPACE}/src/leetcode/${modname}/mod.rs`;
    const lc_code_path = `${CURRENT_WORKSPACE}/src/leetcode/${modname}/solution.rs`;
    ps.execSync(`touch ${lc_mod_path} ${lc_code_path}`);
    fs.writeFileSync(lc_mod_path, '#![allow(unused)]');
    fs.writeFileSync(lc_mod_path, 'pub mod solution;');
    fs.writeFileSync(lc_code_path, 'pub struct Solution;\n\nimpl Solution {\n}\n');
    fs.appendFileSync(LC_MOD_FILE_ROOT, `\npub mod ${modname};`)
}


run();
