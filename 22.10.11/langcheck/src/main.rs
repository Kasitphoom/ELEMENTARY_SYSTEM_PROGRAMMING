use std::{vec, env, fs};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Lang {Japanese, English, Chinese, Burmese, Korea, AccentedLatin, ASCII}

#[derive(Debug, Clone)]
struct LangType{
    lang: Lang,
    uniup: u64,
    unidown: u64,
    text: String,
}

fn find_unicode(langtype:[LangType;7], c:char) -> Lang{
    let mut lang = Lang::AccentedLatin;
    let uni = c as u64;
    let mut found = false;
    for i in 0..7{
        if uni <= langtype[i].uniup && uni >= langtype[i].unidown{
            lang = langtype[i].lang;
            found = true;
            break;
        }
    }
    if !found {
       panic!("The letter {} not found in any cases ({:x})", c, uni); 
    }

    lang
}

fn returnlang(lang:Lang) -> String{
    let mut langstr = String::new();
    match lang{
        Lang::Japanese => langstr = String::from("Japanese"),
        Lang::English => langstr = String::from("English"),
        Lang::Chinese => langstr = String::from("Chinese"),
        Lang::Burmese => langstr = String::from("Burmese"),
        Lang::Korea => langstr = String::from("Korean"),
        Lang::AccentedLatin => langstr = String::from("Accented Latin"),
        Lang::ASCII => langstr = String::from("ASCII"),
    }
    langstr
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let text = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let lines = text.lines();

    // make an array of struct for each language
    let langtype = [
        LangType{lang: Lang::Japanese, unidown: 0x3040, uniup: 0x309F, text: "Japanese".to_string()},
        LangType{lang: Lang::English, unidown: 0x0041, uniup: 0x007A, text: "English".to_string()},
        LangType{lang: Lang::Chinese, unidown: 0x4E00, uniup: 0x9FFF, text: "Chinese".to_string()},
        LangType{lang: Lang::Burmese, unidown: 0x1000, uniup: 0x109F, text: "Burmese".to_string()},
        LangType{lang: Lang::Korea, unidown: 0xAC00, uniup: 0xD7AF, text: "Korea".to_string()},
        LangType{lang: Lang::AccentedLatin, unidown: 0x00C0, uniup: 0x00FF, text: "Accented Latin".to_string()},
        LangType{lang: Lang::ASCII, unidown: 0x0020, uniup: 0x007E, text: "ASCII".to_string()},
    ];

    let mut i = 0;
    for line in lines{
        print!("Line {}: ", i + 1);
        let mut langvec = vec![];
        let mut freqvec = vec![];
        for c in line.chars(){
            let lang = find_unicode(langtype.clone(), c);
            if langvec.contains(&lang){
                let index = langvec.iter().position(|&r| r == lang).unwrap();
                freqvec[index] += 1;
            }
            else{
                langvec.push(lang);
                freqvec.push(1);
            }
        }
        for i in 0..langvec.len(){
            print!("{} ({}), ", returnlang(langvec[i]), freqvec[i]);
        }
        println!("");
        i += 1;
    }

}
