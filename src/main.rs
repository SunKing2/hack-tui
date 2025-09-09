fn ed_main() {
    // Header bar
    println!("\x1b[1;37;44m─────────────────────────[ hack-tui Demo ]───────────────────────\x1b[0m");

    // Fake scrollable text area
    println!("\x1b[0;37;40m┌───────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[0;37;40m│ You can:                                                      │\x1b[0m");
    println!("\x1b[0;37;40m│ Deploy me to the web to with one simple command!:             │\x1b[0m");
    println!("\x1b[0;37;40m│ $ zonks blorb                                                 │\x1b[0m");
    println!("\x1b[0;37;40m│ …                                                             │\x1b[0m");
    println!("\x1b[0;37;40m│ …                                                             │\x1b[0m");
    println!("\x1b[0;37;40m└───────────────────────────────────────────────────────────────┘\x1b[0m");

    // Input line
    println!("\x1b[0;37;40m> Type your one-liner here…\x1b[0m");

    // Buttons row
    println!();
    println!("\x1b[1;30;42m[ Submit ]\x1b[0m   \x1b[1;37;41m[ Cancel ]\x1b[0m   \x1b[1;37;45m[ Surprise! ]\x1b[0m");
}


fn rmain() {
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


fn main() {


    println!();
    println!();
    println!();
    ed_main();
    println!();
    println!();
    println!();


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
    


    // Floating piece
    println!("             {}", purple);
    println!("           {}{}", purple, purple);
    println!("             {}", purple);
    println!();

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


