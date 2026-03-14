use std::process;

const N_STACK: usize = 1000;
const S_STACK: usize = 1000;
const BUFF_LEN: usize = 100;

static mut BUFF_ARR: [char; BUFF_LEN] = ['.'; BUFF_LEN];
static mut BUFF_TOP: i32 = -1;

static mut NSTACK_ARR: [i32; N_STACK] = [-1; N_STACK];
static mut N_STACK_TOP: i32 = -1;

static mut SSTACK_ARR: [char; S_STACK] = ['.'; S_STACK];
static mut S_STACK_TOP: i32 = -1;

fn unget_ch(c: char) {

    unsafe {
        BUFF_TOP += 1;

        if BUFF_TOP as usize >= BUFF_LEN {
            println!("BUFFER Overflow");
        } else {
            BUFF_ARR[BUFF_TOP as usize] = c;
        }

    }

}

fn get_ch() -> char {

    unsafe {
        BUFF_TOP -= 1;

        if BUFF_TOP > -1 {
            return BUFF_ARR[BUFF_TOP as usize];
        }

    }

    '.'
}

fn push_number(num: i32) {

}

fn pop_number() -> i32 {
    0
}

fn calculate(symbol: char) {

    match symbol {
        '+' => {
            push_number(pop_number() + pop_number());
        }

        '-' => {
            let f_n: i32 = pop_number();
            push_number(pop_number() - f_n);
        }

        '*' => {
            push_number(pop_number() * pop_number());
        }

        '/' => {
            let f_n: i32 = pop_number();

            if f_n == 0 {
                println!("Err 0 cannot be divisor");
                process::exit(1);
            }
        }
        _ => {
            println!("The symbol is not expected");
        }
    }
    
}

fn main() {
}