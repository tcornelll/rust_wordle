use std::io;
use rand::Rng;
use std::{thread, time};

fn main() {

    println!("WORDLE");

    let words = "about alert argue beach above alike arise began abuse alive array begin actor allow aside begun acute alone asset being admit along audio below adopt alter audit bench adult among avoid billy after anger award birth again angle aware black agent angry badly blame agree apart baker blind ahead apple bases block alarm apply basic blood album arena basis board boost buyer china cover booth cable chose craft bound calif civil crash brain carry claim cream brand catch class crime bread cause clean cross break chain clear crowd breed chair click crown brief chart clock curve bring chase close cycle broad cheap coach daily broke check coast dance brown chest could dated build chief count dealt built child court death debut entry fourth group delay equal forty grown depth error forum guard doing event found guess doubt every frame guest dozen exact frank guide draft exist fraud happy drama extra fresh harry drawn faith front heart dream false fruit heavy dress fault fully hence drill fibre funny night drink field giant horse drive fifth given hotel drove fifty glass house dying fight globe human eager final going ideal early first grace image earth fixed grade index eight flash grand inner elite fleet grant input empty floor grass issue enemy fluid great irony enjoy focus green juice enter force gross joint judge metal media newly known local might noise label logic minor north large loose minus noted laser lower mixed novel later lucky model nurse laugh lunch money occur layer lying month ocean learn magic moral offer lease major motor often least maker mount order leave march mouse other legal music mouth ought level match movie paint light mayor needs paper limit meant never party peace power radio round panel press raise route phase price range royal phone pride rapid rural photo prime ratio scale piece print reach scene pilot prior ready scope pitch prize refer score place proof right sense plain proud rival serve plane prove river seven plant queen quick shall plate sixth stand shape point quiet roman share pound quite rough sharp sheet spare style times shelf speak sugar tired shell speed suite title shift spend super today shirt spent sweet topic shock split table total shoot spoke taken touch short sport taste tough shown staff taxes tower sight stage teach track since stake teeth trade sixty start texas treat sized state thank trend skill steam theft trial sleep steel their tried slide stick theme tries small still there truck smart stock these truly smile stone thick trust smith stood thing  truth smoke sotre think twice solid storm third under solve story those undue sorry strip three union sound stuck threw unity south study throw until space stuff tight upper upset whole waste wound urban whose watch write usage woman water wrong usual train wheel wrote valid world where yield value worry which young video worse while youth virus worst white worth visit would vital voice";

    let word_array: Vec<&str> = words.split(" ").filter(|s| s.len() == 5).collect();
    let letter_wait_time = time::Duration::from_millis(500);
    let line_wait_time = time::Duration::from_secs(1);

    'game_loop: loop {
        let secret_word = word_array[rand::thread_rng().gen_range(0..word_array.len())];
        let mut guesses_left = 6;
        let mut displayed_word: [char; 5] = ['*'; 5];
        let mut guessed_chars: Vec<char> = Vec::new();
        'round_loop: while guesses_left > 0 {
            //displaying the word, with stars for unguessed letters
            for i in 0..displayed_word.len(){
                print!("{}    ", displayed_word[i]);
                thread::sleep(letter_wait_time);
            }
            println!();

            //loop to read the chosen letter
            let chosen_char = loop{
                let mut guess = String::new();
                thread::sleep(line_wait_time);
                println!("Guess a letter of the word");
                thread::sleep(line_wait_time);
                println!("You have {} incorrect guesses left", guesses_left);
                thread::sleep(line_wait_time);
                println!("/ to quit");
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
                let guess: char = match guess.trim().parse() {
                    Ok(char) => char,
                    Err(_) => {
                        println!("Please guess a letter!");
                        thread::sleep(line_wait_time);
                        continue;
                    }
                };
                if guess == '/' {break 'game_loop;}
                break guess;
            };

            //check if the chosen character is a letter
            if !chosen_char.is_alphabetic(){
                println!("Please guess a letter!");
                thread::sleep(line_wait_time);
                continue;
            }

            //check if user has guessed that letter already
            if guessed_chars.contains(&chosen_char) {
                println!("You've already guessed that letter!");
                thread::sleep(line_wait_time);
                continue;
            }

            guessed_chars.push(chosen_char);

            //if the secret word contains the guessed letter
            if secret_word.contains(chosen_char) {
                let mut char_indices: Vec<usize> = Vec::new();
                for i in 0..secret_word.len() {
                    if chosen_char == secret_word.chars().nth(i).unwrap() {
                        char_indices.push(i);
                    }
                }
                for idx in char_indices {
                    displayed_word[idx] = chosen_char;
                }
            }
            //if it doesn't
            else {
                guesses_left -= 1;
                println!("Womp Womp! The word doesn't contain that letter!");
                thread::sleep(line_wait_time);
            }
            //checking if user has won
            let unguessed_chars = displayed_word.iter().copied().filter(|&char| char != '*').count();
            if unguessed_chars == secret_word.len() {
                println!("You win! The word was {}", secret_word);
                thread::sleep(line_wait_time);
                break 'round_loop;
            }
            //if the user has lost (ran out of incorrect guesses)
            if guesses_left == 0 {
                println!("You lose! The word was {}", secret_word);
                thread::sleep(line_wait_time);
            }
        }

    }

}
