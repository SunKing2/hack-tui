fn oldmain() {
    // Header bar
    println!("\x1b[1;37;44m─────────────────────────[ Stellar Demo ]─────────────────────────\x1b[0m");

    // Fake scrollable text area
    println!("\x1b[0;37;40m┌───────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[0;37;40m│ Lorem ipsum dolor sit amet, consectetur adipiscing elit.      │\x1b[0m");
    println!("\x1b[0;37;40m│ Sed do eiusmod tempor incididunt ut labore et dolore magna.   │\x1b[0m");
    println!("\x1b[0;37;40m│ Ut enim ad minim veniam, quis nostrud exercitation ullamco.   │\x1b[0m");
    println!("\x1b[0;37;40m│ laboris nisi ut aliquip ex ea commodo consequat.              │\x1b[0m");
    println!("\x1b[0;37;40m│ …                                                            │\x1b[0m");
    println!("\x1b[0;37;40m└───────────────────────────────────────────────────────────────┘\x1b[0m");

    // Input line
    println!("\x1b[0;37;40m> Type your one-liner here…\x1b[0m");

    // Buttons row
    println!();
    println!("\x1b[1;30;42m[ Submit ]\x1b[0m   \x1b[1;37;41m[ Cancel ]\x1b[0m   \x1b[1;37;45m[ Surprise! ]\x1b[0m");
}


fn old2main() {
    // Top border
    println!("\x1b[1;37;40m┌──────────────┐\x1b[0m");

    // Empty rows
    for _ in 0..8 {
        println!("\x1b[1;37;40m│              │\x1b[0m");
    }

    // One cyan Tetris block
    println!("\x1b[1;37;40m│   \x1b[1;37;46m    \x1b[0m     │\x1b[0m");
    println!("\x1b[1;37;40m│   \x1b[1;37;46m    \x1b[0m     │\x1b[0m");
    println!("\x1b[1;37;40m│   \x1b[1;37;46m    \x1b[0m     │\x1b[0m");
    println!("\x1b[1;37;40m│   \x1b[1;37;46m    \x1b[0m     │\x1b[0m");

    // Bottom border
    println!("\x1b[1;37;40m└──────────────┘\x1b[0m");
}

fn fake3main() {
    let cyan = "\x1b[1;37;46m  \x1b[0m";
    let yellow = "\x1b[1;30;43m  \x1b[0m";

    // I-bar
    println!("{}", cyan);
    println!("{}", cyan);
    println!("{}", cyan);
    print!("{}", cyan);print!("{}", cyan);println!("{}", cyan);

    println!(); // spacer

    // O-block
    println!("{}{}", yellow, yellow);
    println!("{}{}", yellow, yellow);
}

fn main() {

/*
    // --- Standard ANSI Colors (theme-dependent) ---
    let i_block = "\x1b[46m  \x1b[0m"; // Cyan background
    let j_block = "\x1b[44m  \x1b[0m"; // Blue background
    let l_block = "\x1b[43m  \x1b[0m"; // Yellow-ish (may look brown/orange)
    let o_block = "\x1b[103m  \x1b[0m"; // Bright yellow (fallback)
    let s_block = "\x1b[42m  \x1b[0m"; // Green background
    let t_block = "\x1b[45m  \x1b[0m"; // Magenta/purple background
    let z_block = "\x1b[41m  \x1b[0m"; // Red background
*/

    // --- TrueColor (24-bit RGB, exact Tetris colors) ---
    let i_block = "\x1b[48;2;0;255;255m  \x1b[0m";   // Cyan
    let j_block = "\x1b[48;2;0;0;255m  \x1b[0m";     // Blue
    let l_block = "\x1b[48;2;255;165;0m  \x1b[0m";   // Orange
    let o_block = "\x1b[48;2;255;255;0m  \x1b[0m";   // Yellow
    let s_block = "\x1b[48;2;0;255;0m  \x1b[0m";     // Green
    let t_block = "\x1b[48;2;128;0;128m  \x1b[0m";   // Purple
    let z_block = "\x1b[48;2;255;0;0m  \x1b[0m";     // Red
    
    let cyan    = i_block;
    let blue    = j_block;
    let orange  = l_block;
    let yellow  = o_block;
    let green   = s_block;
    let purple  = t_block;
    let red     = z_block;
    


/*
    // these are good colors so I'll keep them in case the earlier defs don't work
    // Define block "pixels"
    let cyan   = "\x1b[1;37;46m  \x1b[0m"; // I
    let green  = "\x1b[1;30;42m  \x1b[0m"; // S
    let purple = "\x1b[1;37;45m  \x1b[0m"; // T
    let red    = "\x1b[1;37;41m  \x1b[0m"; // Z
    let yellow = "\x1b[48;2;255;255;0m  \x1b[0m"; // RGB yellow block
*/



    // Floating piece
    println!("             {}", purple);
    println!("           {}{}", purple, purple);
    println!("             {}", purple);
    println!();


/*
    // some blue and orange trials
    println!("             {}{}", blue, blue);
    println!("             {}", blue);
    println!("             {}", blue);
    println!();

    println!("             {}", orange);
    println!("             {}", orange);
    println!("             {}{}", orange, orange);
    println!();
*/    


/*
    // Row 1 (cyan I)
    println!("{} ", cyan);
    println!("{} ", cyan);
    println!("{} ", cyan);
    println!("{} ", cyan);

    // Row 2 (green S)
    println!("        {}{}", green, green);
    println!("      {}{}", green, green);

    // Row 3 (purple inverted T)
    println!("                {}", purple);
    println!("              {}{}{}", purple, purple, purple);

    // Row 4 (red Z)
    println!("                      {}{}", red, red);
    println!("                        {}{}", red, red);
    
*/    


    // experiment
    // Print cyan block, then return to start of line and print green block offset

    println!();
    print!("{}", cyan);
    println!();

    print!("{}", cyan);
    println!();

    print!("{}", cyan);
    print!("      {}{}", green, green);
    print!("      {}", orange);
    print!("  {}{}", red, red);
    println!();
    
    print!("{}", cyan);
    print!("    {}{}", green, green);
    print!("    {}{}{}", orange, orange, orange);
    print!("    {}{}", red, red);
    println!();
    println!();
}


