pub mod qmcoder {

    #[allow(non_snake_case)]
    #[derive(Debug)]
    struct QMstatus {
        State:u8,
        qcHex:u32,
        qcDec:f32,
        In:u8, // increase
        De:u8, // decrease
    }
    impl QMstatus {
        pub fn new(State:u8, qcHex:u32, qcDec:f32, In:u8, De:u8)->QMstatus {
            QMstatus {
                State,
                qcHex,
                qcDec,
                In,
                De,
            }
        }
    }

    #[allow(non_snake_case)]
    pub struct Encoder {
        qm_table: Vec<QMstatus>,
        state:u8,
        Qc:u32,
        A:u32 ,
        c:u32,
        CT:u8,    // C&A left shift count
        SC:u8,    // stack count
        BP:u32,
        MPS:bool,
        LPS:bool,
    }
    impl Encoder {
        pub fn new()-> Encoder {
            Encoder {
                qm_table: Vec::new(),
                state:0,
                Qc: 0x59EB,
                A: 0x10000,
                c: 0x8000,
                CT: 11,  
                SC: 0,         
                BP: 0,
                MPS: true,
                LPS: false,
            }
        }
        pub fn read_QT_table(mut self) {
            use std::io::{BufRead, BufReader};
            use std::fs::File;

            let reader = BufReader::new(File::open("src/qmtable.txt").expect("Cannot open qmtable.txt"));

            let mut cnt = 0;
            let mut state:u8 = 0;
            let mut qcHex:u32 = 0; 
            let mut qcDec:f32 = 0.0;
            let mut In:u8 = 0;
            let mut De:u8 = 0;
            for line in reader.lines() {
                for word in line.unwrap().split_whitespace() {
                    // println!("word '{}' cnt:{}", word, cnt);
                    match cnt {
                        0 => { state = word.parse::<u8>().unwrap(); }
                        1 => { qcHex = u32::from_str_radix(word, 16).unwrap(); }
                        2 => { qcDec = word.parse::<f32>().unwrap(); }
                        3 => { In = word.parse::<u8>().unwrap(); }
                        4 => {
                            match word.parse::<u8>() {
                                Ok(n) => De = n,
                                Err(e) => {
                                    println!("ErrorMsg: {}",e);
                                    De = 0;
                                },
                            };
                            let tmp = QMstatus::new(state, qcHex, qcDec, In, De);
                            // println!("{:?}",&tmp);
                            self.qm_table.push(tmp);             
                        }
                        _ => {panic!("Unexpected error at match cnt!");}
                    }
                    cnt = (cnt + 1)%5;
                }
            }
        }
        pub fn change_state(){}
        pub fn renormalize(mut self, result:&Vec<u8>){
            while self.A < 0x8000 {
                self.A <<= 1;
                self.c <<= 1;
                self.CT -= 1;

                if self.CT == 0 {
                    // byte out
                    let t = self.c >> 19;

                    if t > 0xff {
                        self.BP += 1;
                        // stuff 0
                        if self.BP == 0xff {
                            // result += '{0:b}'.format(self.BP)
                            self.BP = 0;
                        }
                        // output stacked zeros
                        while self.SC > 0 {
                           //result += '{0:b}'.format(self.BP)
                           self.BP = 0;
                           self.SC -= 1; 
                        }
                        //result += '{0:b}'.format(self.BP)
                        self.BP = t;
                    }
                    else {
                        if t == 0xff {
                            self.SC += 1;
                        } else {
                            while self.SC > 0 {
                                // result += '{0:b}'.format(self.BP)
                                self.BP = 0xff;
                                // result += '{0:b}'.format(self.BP)
                                self.BP = 0;
                                self.SC -= 1;
                            }
                            // result += '{0:b}'.format(self.BP)
                            self.BP = t;
                        }   
                    }
                    self.c &= 0x7ffff;
                    self.CT = 8;
                }
            }    
            //result
        }
        
        pub fn encode(mut self, bit_string: Vec<bool>)->Vec<u8> {
            let mut result = Vec::new();

            for bit in bit_string {
                if bit == self.MPS {
                    self.A = self.A - self.Qc;
                    if self.A < 0x8000 {
                        if self.A < self.Qc {
                            self.c += self.A;
                            self.A = self.Qc;
                        }
                    // change Qn stage
                    // self.change_state();
                    // renormalize
                    // result =  self.renormalize(result);
                    }
                }
                if bit == self.LPS {
                    self.A = self.A - self.Qc;
                    if self.A >= self.Qc {
                        self.c += self.A;
                        self.A = self.Qc;
                    }
                    // change Qn stage
                    // self.change_state();
                    // renormalize
                    // self.renormalize();  
                }
            }
        result
        }
    }
}