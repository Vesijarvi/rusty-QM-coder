pub mod qmcoder {
    extern crate csv;
    use std::fs::File;

    #[allow(non_snake_case)]
    struct QMstatus {
        State:u8,
        qcHex:u32,
        qcDec:u32,
        In:u8, // increase
        De:u8, // decrease
    }

    #[allow(non_snake_case)]
    struct Encoder {
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
        fn new()-> Encoder {
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
        fn read_QT_table(self) {
            let qmTableFile = File::open("qmtable.txt").expect("Could not open qmtable.txt");

        }
        fn change_state(){}
        fn renormalize(mut self, result:&Vec<u8>){
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