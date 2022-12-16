const { resolve } = require('path');
const { readdirSync } = require('fs');
const { exec, execSync } = require('child_process');

const workspace_root = resolve(__dirname, '..');
function print_red_bold_message(text) {
  console.log(`\x1b[31m\x1b[1m${text}\x1b[0m`);
}
function print_green_bold_message(text) {
  console.log(`\x1b[32m\x1b[1m${text}\x1b[0m`);
}
function print_blue_bold_message(text) {
  console.log(`\x1b[34m\x1b[1m${text}\x1b[0m`);
}

// start antlrv4-parse check
const antlr4_parse_check = () => {
  const samples = readdirSync(resolve(workspace_root, 'examples/src'))
  let pass_check = true;
  for (const file_name of samples) {
    if (file_name.endsWith('.n')) {
      const returns = execSync('antlr4-parse ' + [
        'specs/NebulaParser.g4',
        "entry_file",
        `examples/src/${file_name}`,
      ].join(' '), { cwd: workspace_root, encoding: 'utf8' });
      if (returns.length > 0) {
        print_red_bold_message(`[ERROR] antlr check ${file_name} failed!`);
        console.log(`\t${returns}`);
        pass_check = false;
      } else {
        print_green_bold_message(`[INFO] antlr check ${file_name} succeed!`);
      }
    }
  }
  if (!pass_check) {
    process.exit(1);
  }
}
const cargo_test_callback = (error, stdout) => {
  if (error) {
    print_red_bold_message('[ERROR] cargo test failed!');
    console.log(`\t${error}`);
    process.exit(1);
  } else {
    print_green_bold_message('[INFO] cargo test succeed!');
    console.log(stdout);

    // start antlr4-parse check
    print_blue_bold_message('[INFO] start antlr4-parse check...');
    antlr4_parse_check();
  }
}
const cargo_check_callback = (error) => {
  if (error) {
    print_red_bold_message('[ERROR] cargo check failed!');
    console.log(`\t${error}`);
    process.exit(1);
  } else {
    print_green_bold_message('[INFO] cargo check succeed!');

    // start cargo test
    print_blue_bold_message('[INFO] start cargo test...');
    exec('cargo test', { shell: true }, cargo_test_callback);
  }
}

// start cargo check
print_blue_bold_message('[INFO] start cargo check...');
exec('cargo check', { shell: true },cargo_check_callback);
