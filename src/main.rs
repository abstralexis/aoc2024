//! Main method thingy. I might use it for testing, might not.
//! The *Main* point is allowing me to put all solutions in one
//! crate and getting nice syntax highlighting. Isn't that wonderful? 
// use day1;
// use day2;
// use day3;
use day5;

fn main() {
    println!(r###"
         ________  ________  ___      ___ _______   ________   _________        ________  ________          
        |\   __  \|\   ___ \|\  \    /  /|\  ___ \ |\   ___  \|\___   ___\     |\   __  \|\  _____\        
        \ \  \|\  \ \  \_|\ \ \  \  /  / | \   __/|\ \  \\ \  \|___ \  \_|     \ \  \|\  \ \  \__/        
         \ \   __  \ \  \ \\ \ \  \/  / / \ \  \_|/_\ \  \\ \  \   \ \  \       \ \  \\\  \ \   __\      
          \ \  \ \  \ \  \_\\ \ \    / /   \ \  \_|\ \ \  \\ \  \   \ \  \       \ \  \\\  \ \  \_|     
           \ \__\ \__\ \_______\ \__/ /     \ \_______\ \__\\ \__\   \ \__\       \ \_______\ \__\     
            \|__|\|__|\|_______|\|__|/       \|_______|\|__| \|__|    \|__|        \|_______|\|__|     
                                                                                                                                                                                             
 ________  ________  ________  _______            _______  ________    _______  ___   ___               .      .
|\   ____\|\   __  \|\   ___ \|\  ___ \          /  ___  \|\   __  \  /  ___  \|\  \ |\  \              _\/  \/_
\ \  \___|\ \  \|\  \ \  \_|\ \ \   __/|        /__/|_/  /\ \  \|\  \/__/|_/  /\ \  \\_\  \              _\/\/_
 \ \  \    \ \  \\\  \ \  \ \\ \ \  \_|/__      |__|//  / /\ \  \\\  \__|//  / /\ \______  \         _\_\_\/\/_/_/_
  \ \  \____\ \  \\\  \ \  \_\\ \ \  \_|\ \         /  /_/__\ \  \\\  \  /  /_/__\|_____|\  \         / /_/\/\_\ \
   \ \_______\ \_______\ \_______\ \_______\       |\________\ \_______\|\________\     \ \__\           _/\/\_
    \|_______|\|_______|\|_______|\|_______|        \|_______|\|_______| \|_______|      \|__|           /\  /\
                                                                                              
                                                                                                                                                                                                                                                                                                                                   
    "###);

    // day1::part1();
    // day1::part2();

    // day2::part1();
    // day2::part2();

    // slow!
    // day3::part1();
    // day3::part2();

    // day5::part1();
    day5::part2();
}
