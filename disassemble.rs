// followed this guide on creating an 8080 disassembler: http://www.emulator101.com/disassembler-pt-1.html
use std::fs;
use std::env;

// Turns an array of bytes containing the hexdump into a string
fn disassemble(hex: &[u8]) -> String {
    let mut asm = "".to_owned();
    let mut buffer = "".to_owned();

    for i in 0..hex.len() {
        if hex[i] as char == ' ' || hex[i] as char == '\n' {
            continue;
        }
        else {
            // First 7 characters of each line (of 48 characters) is just the line number
            if i % 48 < 7 {
                continue;
            } else {
                buffer.push_str(&(hex[i] as char).to_string());
            }
        }

        // Ensure that we have 6 hex characters before disassembling
        if buffer.chars().count() < 6 {
            continue;
        }

        if buffer.chars().count() > 6 {
            println!("FATAL ERROR: buffer is full");
            return "".to_owned();
        }

        match &buffer[0..2] {
            // Used as reference: http://www.emulator101.com/reference/8080-by-opcode.html
            "00" => {
                asm.push_str(&"NOP\n");
                buffer = buffer[2..6].to_owned();
            },            "01" => {
                asm.push_str(&format!("LXI    B,#${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "02" => {
                asm.push_str(&"STAX   B\n");
                buffer = buffer[2..6].to_owned();
            },            "03" => {
                asm.push_str(&"INX    B\n");
                buffer = buffer[2..6].to_owned();
            },            "04" => {
                asm.push_str(&"INR    B\n");
                buffer = buffer[2..6].to_owned();
            },            "05" => {
                asm.push_str(&"DCR    B\n");
                buffer = buffer[2..6].to_owned();
            },            "06" => {
                asm.push_str(&format!("MVI    B,#${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "07" => {
                asm.push_str(&"RCL    B\n");
                buffer = buffer[2..6].to_owned();
            },            "08" => {
                buffer = buffer[2..6].to_owned();
            },            "09" => {
                asm.push_str(&"DAD    B\n");
                buffer = buffer[2..6].to_owned();
            },            "0a" => {
                asm.push_str(&"LDAX   B\n");
                buffer = buffer[2..6].to_owned();
            },            "0b" => {
                asm.push_str(&"DCX    B\n");
                buffer = buffer[2..6].to_owned();
            },            "0c" => {
                asm.push_str(&"INR    C\n");
                buffer = buffer[2..6].to_owned();
            },            "0d" => {
                asm.push_str(&"DCR    C\n");
                buffer = buffer[2..6].to_owned();
            },            "0e" => {
                asm.push_str(&format!("MVI    C,#${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "0f" => {
                asm.push_str(&"RRC\n");
                buffer = buffer[2..6].to_owned();
            },            "10" => {
                buffer = buffer[2..6].to_owned();
            },            "11" => {
                asm.push_str(&format!("LXI    D,#${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "12" => {
                asm.push_str(&"STAX   D\n");
                buffer = buffer[2..6].to_owned();
            },            "13" => {
                asm.push_str(&"INX    D\n");
                buffer = buffer[2..6].to_owned();
            },            "14" => {
                asm.push_str(&"INR    D\n");
                buffer = buffer[2..6].to_owned();
            },            "15" => {
                asm.push_str(&"DCR    D\n");
                buffer = buffer[2..6].to_owned();
            },            "16" => {
                asm.push_str(&format!("CPI    D,#${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "17" => {
                asm.push_str(&"RAL\n");
                buffer = buffer[2..6].to_owned();
            },            "18" => {
                buffer = buffer[2..6].to_owned();
            },            "19" => {
                asm.push_str(&"DAD    D\n");
                buffer = buffer[2..6].to_owned();
            },            "1a" => {
                asm.push_str(&"LDAX   D\n");
                buffer = buffer[2..6].to_owned();
            },            "1b" => {
                asm.push_str(&"DCX    D\n");
                buffer = buffer[2..6].to_owned();
            },            "1c" => {
                asm.push_str(&"INR    E\n");
                buffer = buffer[2..6].to_owned();
            },            "1d" => {
                asm.push_str(&"DCR    E\n");
                buffer = buffer[2..6].to_owned();
            },            "1e" => {
                asm.push_str(&format!("MVI    E,#${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "1f" => {
                asm.push_str(&"RAR\n");
                buffer = buffer[2..6].to_owned();
            },            "20" => {
                buffer = buffer[2..6].to_owned();
            },            "21" => {
                asm.push_str(&format!("LXI    H,#${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "22" => {
                asm.push_str(&format!("SHLD   ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "23" => {
                asm.push_str(&"INX    H\n");
                buffer = buffer[2..6].to_owned();
            },            "24" => {
                asm.push_str(&"INR    H\n");
                buffer = buffer[2..6].to_owned();
            },            "25" => {
                asm.push_str(&"DCR    H\n");
                buffer = buffer[2..6].to_owned();
            },            "26" => {
                asm.push_str(&format!("MVI    H,#${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "27" => {
                asm.push_str(&"DAA\n");
                buffer = buffer[2..6].to_owned();
            },            "28" => {
                buffer = buffer[2..6].to_owned();
            },            "29" => {
                asm.push_str(&"DAD    H\n");
                buffer = buffer[2..6].to_owned();
            },            "2a" => {
                asm.push_str(&format!("LHLD   ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "2b" => {
                asm.push_str(&"DCX    H\n");
                buffer = buffer[2..6].to_owned();
            },            "2c" => {
                asm.push_str(&"INR    L\n");
                buffer = buffer[2..6].to_owned();
            },            "2d" => {
                asm.push_str(&"DCR    L\n");
                buffer = buffer[2..6].to_owned();
            },            "2e" => {
                asm.push_str(&format!("MVI    L,#${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "2f" => {
                asm.push_str(&"CMA\n");
                buffer = buffer[2..6].to_owned();
            },            "30" => {
                buffer = buffer[2..6].to_owned();
            },            "31" => {
                asm.push_str(&format!("LXI    SP,#${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "32" => {
                asm.push_str(&format!("STA    ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "33" => {
                asm.push_str(&"INX    SP\n");
                buffer = buffer[2..6].to_owned();
            },            "34" => {
                asm.push_str(&"INR    M\n");
                buffer = buffer[2..6].to_owned();
            },            "35" => {
                asm.push_str(&"DCR    M\n");
                buffer = buffer[2..6].to_owned();
            },            "36" => {
                asm.push_str(&format!("MVI    M,#${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "37" => {
                asm.push_str(&"STC\n");
                buffer = buffer[2..6].to_owned();
            },            "38" => {
                buffer = buffer[2..6].to_owned();
            },            "39" => {
                asm.push_str(&"DAD    SP\n");
                buffer = buffer[2..6].to_owned();
            },            "3a" => {
                asm.push_str(&format!("LDA    ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "3b" => {
                asm.push_str(&"DCX    SP\n");
                buffer = buffer[2..6].to_owned();
            },            "3c" => {
                asm.push_str(&"INR    A\n");
                buffer = buffer[2..6].to_owned();
            },            "3d" => {
                asm.push_str(&"DCR    A\n");
                buffer = buffer[2..6].to_owned();
            },            "3e" => {
                asm.push_str(&format!("MVI    A,#${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "3f" => {
                asm.push_str(&"CMC\n");
                buffer = buffer[2..6].to_owned();
            },            "40" => {
                asm.push_str(&"MOV    B,B\n");
                buffer = buffer[2..6].to_owned();
            },            "41" => {
                asm.push_str(&"MOV    B,C\n");
                buffer = buffer[2..6].to_owned();
            },            "42" => {
                asm.push_str(&"MOV    B,D\n");
                buffer = buffer[2..6].to_owned();
            },            "43" => {
                asm.push_str(&"MOV    B,E\n");
                buffer = buffer[2..6].to_owned();
            },            "44" => {
                asm.push_str(&"MOV    B,H\n");
                buffer = buffer[2..6].to_owned();
            },            "45" => {
                asm.push_str(&"MOV    B,L\n");
                buffer = buffer[2..6].to_owned();
            },            "46" => {
                asm.push_str(&"MOV    B,M\n");
                buffer = buffer[2..6].to_owned();
            },            "47" => {
                asm.push_str(&"MOV    B,A\n");
                buffer = buffer[2..6].to_owned();
            },            "48" => {
                asm.push_str(&"MOV    C,B\n");
                buffer = buffer[2..6].to_owned();
            },            "49" => {
                asm.push_str(&"MOV    C,C\n");
                buffer = buffer[2..6].to_owned();
            },            "4a" => {
                asm.push_str(&"MOV    C,D\n");
                buffer = buffer[2..6].to_owned();
            },            "4b" => {
                asm.push_str(&"MOV    C,E\n");
                buffer = buffer[2..6].to_owned();
            },            "4c" => {
                asm.push_str(&"MOV    C,H\n");
                buffer = buffer[2..6].to_owned();
            },            "4d" => {
                asm.push_str(&"MOV    C,L\n");
                buffer = buffer[2..6].to_owned();
            },            "4e" => {
                asm.push_str(&"MOV    C,M\n");
                buffer = buffer[2..6].to_owned();
            },            "4f" => {
                asm.push_str(&"MOV    C,A\n");
                buffer = buffer[2..6].to_owned();
            },            "50" => {
                asm.push_str(&"MOV    D,B\n");
                buffer = buffer[2..6].to_owned();
            },            "51" => {
                asm.push_str(&"MOV    D,C\n");
                buffer = buffer[2..6].to_owned();
            },            "52" => {
                asm.push_str(&"MOV    D,D\n");
                buffer = buffer[2..6].to_owned();
            },            "53" => {
                asm.push_str(&"MOV    D,E\n");
                buffer = buffer[2..6].to_owned();
            },            "54" => {
                asm.push_str(&"MOV    D,H\n");
                buffer = buffer[2..6].to_owned();
            },            "55" => {
                asm.push_str(&"MOV    D,L\n");
                buffer = buffer[2..6].to_owned();
            },            "56" => {
                asm.push_str(&"MOV    D,M\n");
                buffer = buffer[2..6].to_owned();
            },            "57" => {
                asm.push_str(&"MOV    D,A\n");
                buffer = buffer[2..6].to_owned();
            },            "58" => {
                asm.push_str(&"MOV    E,B\n");
                buffer = buffer[2..6].to_owned();
            },            "59" => {
                asm.push_str(&"MOV    E,C\n");
                buffer = buffer[2..6].to_owned();
            },            "5a" => {
                asm.push_str(&"MOV    E,D\n");
                buffer = buffer[2..6].to_owned();
            },            "5b" => {
                asm.push_str(&"MOV    E,E\n");
                buffer = buffer[2..6].to_owned();
            },            "5c" => {
                asm.push_str(&"MOV    E,H\n");
                buffer = buffer[2..6].to_owned();
            },            "5d" => {
                asm.push_str(&"MOV    E,L\n");
                buffer = buffer[2..6].to_owned();
            },            "5e" => {
                asm.push_str(&"MOV    E,M\n");
                buffer = buffer[2..6].to_owned();
            },            "5f" => {
                asm.push_str(&"MOV    E,A\n");
                buffer = buffer[2..6].to_owned();
            },            "60" => {
                asm.push_str(&"MOV    H,B\n");
                buffer = buffer[2..6].to_owned();
            },            "61" => {
                asm.push_str(&"MOV    H,C\n");
                buffer = buffer[2..6].to_owned();
            },            "62" => {
                asm.push_str(&"MOV    H,D\n");
                buffer = buffer[2..6].to_owned();
            },            "63" => {
                asm.push_str(&"MOV    H,E\n");
                buffer = buffer[2..6].to_owned();
            },            "64" => {
                asm.push_str(&"MOV    H,H\n");
                buffer = buffer[2..6].to_owned();
            },            "65" => {
                asm.push_str(&"MOV    H,L\n");
                buffer = buffer[2..6].to_owned();
            },            "66" => {
                asm.push_str(&"MOV    H,M\n");
                buffer = buffer[2..6].to_owned();
            },            "67" => {
                asm.push_str(&"MOV    H,A\n");
                buffer = buffer[2..6].to_owned();
            },            "68" => {
                asm.push_str(&"MOV    L,B\n");
                buffer = buffer[2..6].to_owned();
            },            "69" => {
                asm.push_str(&"MOV    L,C\n");
                buffer = buffer[2..6].to_owned();
            },            "6a" => {
                asm.push_str(&"MOV    L,D\n");
                buffer = buffer[2..6].to_owned();
            },            "6b" => {
                asm.push_str(&"MOV    L,E\n");
                buffer = buffer[2..6].to_owned();
            },            "6c" => {
                asm.push_str(&"MOV    L,H\n");
                buffer = buffer[2..6].to_owned();
            },            "6d" => {
                asm.push_str(&"MOV    L,L\n");
                buffer = buffer[2..6].to_owned();
            },            "6e" => {
                asm.push_str(&"MOV    L,M\n");
                buffer = buffer[2..6].to_owned();
            },            "6f" => {
                asm.push_str(&"MOV    L,A\n");
                buffer = buffer[2..6].to_owned();
            },            "70" => {
                asm.push_str(&"MOV    M,B\n");
                buffer = buffer[2..6].to_owned();
            },            "71" => {
                asm.push_str(&"MOV    M,C\n");
                buffer = buffer[2..6].to_owned();
            },            "72" => {
                asm.push_str(&"MOV    M,D\n");
                buffer = buffer[2..6].to_owned();
            },            "73" => {
                asm.push_str(&"MOV    M,E\n");
                buffer = buffer[2..6].to_owned();
            },            "74" => {
                asm.push_str(&"MOV    M,H\n");
                buffer = buffer[2..6].to_owned();
            },            "75" => {
                asm.push_str(&"MOV    M,L\n");
                buffer = buffer[2..6].to_owned();
            },            "76" => {
                asm.push_str(&"HLT\n");
                buffer = buffer[2..6].to_owned();
            },            "77" => {
                asm.push_str(&"MOV    M,A\n");
                buffer = buffer[2..6].to_owned();
            },            "78" => {
                asm.push_str(&"MOV    A,B\n");
                buffer = buffer[2..6].to_owned();
            },            "79" => {
                asm.push_str(&"MOV    A,C\n");
                buffer = buffer[2..6].to_owned();
            },            "7a" => {
                asm.push_str(&"MOV    A,D\n");
                buffer = buffer[2..6].to_owned();
            },            "7b" => {
                asm.push_str(&"MOV    A,E\n");
                buffer = buffer[2..6].to_owned();
            },            "7c" => {
                asm.push_str(&"MOV    A,H\n");
                buffer = buffer[2..6].to_owned();
            },            "7d" => {
                asm.push_str(&"MOV    A,L\n");
                buffer = buffer[2..6].to_owned();
            },            "7e" => {
                asm.push_str(&"MOV    A,M\n");
                buffer = buffer[2..6].to_owned();
            },            "7f" => {
                asm.push_str(&"MOV    A,A\n");
                buffer = buffer[2..6].to_owned();
            },            "80" => {
                asm.push_str(&"ADD    B\n");
                buffer = buffer[2..6].to_owned();
            },            "81" => {
                asm.push_str(&"ADD    C\n");
                buffer = buffer[2..6].to_owned();
            },            "82" => {
                asm.push_str(&"ADD    D\n");
                buffer = buffer[2..6].to_owned();
            },            "83" => {
                asm.push_str(&"ADD    E\n");
                buffer = buffer[2..6].to_owned();
            },            "84" => {
                asm.push_str(&"ADD    H\n");
                buffer = buffer[2..6].to_owned();
            },            "85" => {
                asm.push_str(&"ADD    L\n");
                buffer = buffer[2..6].to_owned();
            },            "86" => {
                asm.push_str(&"ADD    M\n");
                buffer = buffer[2..6].to_owned();
            },            "87" => {
                asm.push_str(&"ADD    A\n");
                buffer = buffer[2..6].to_owned();
            },            "88" => {
                asm.push_str(&"ADC    B\n");
                buffer = buffer[2..6].to_owned();
            },            "89" => {
                asm.push_str(&"ADC    C\n");
                buffer = buffer[2..6].to_owned();
            },            "8a" => {
                asm.push_str(&"ADC    D\n");
                buffer = buffer[2..6].to_owned();
            },            "8b" => {
                asm.push_str(&"ADC    B\n");
                buffer = buffer[2..6].to_owned();
            },            "8c" => {
                asm.push_str(&"ADC    H\n");
                buffer = buffer[2..6].to_owned();
            },            "8d" => {
                asm.push_str(&"ADC    L\n");
                buffer = buffer[2..6].to_owned();
            },            "8e" => {
                asm.push_str(&"ADC    M\n");
                buffer = buffer[2..6].to_owned();
            },            "8f" => {
                asm.push_str(&"ADC    A\n");
                buffer = buffer[2..6].to_owned();
            },            "90" => {
                asm.push_str(&"SUB    B\n");
                buffer = buffer[2..6].to_owned();
            },            "91" => {
                asm.push_str(&"SUB    C\n");
                buffer = buffer[2..6].to_owned();
            },            "92" => {
                asm.push_str(&"SUB    D\n");
                buffer = buffer[2..6].to_owned();
            },            "93" => {
                asm.push_str(&"SUB    E\n");
                buffer = buffer[2..6].to_owned();
            },            "94" => {
                asm.push_str(&"SUB    H\n");
                buffer = buffer[2..6].to_owned();
            },            "95" => {
                asm.push_str(&"SUB    L\n");
                buffer = buffer[2..6].to_owned();
            },            "96" => {
                asm.push_str(&"SUB    M\n");
                buffer = buffer[2..6].to_owned();
            },            "97" => {
                asm.push_str(&"SUB    A\n");
                buffer = buffer[2..6].to_owned();
            },            "98" => {
                asm.push_str(&"SBB    B\n");
                buffer = buffer[2..6].to_owned();
            },            "99" => {
                asm.push_str(&"SBB    C\n");
                buffer = buffer[2..6].to_owned();
            },            "9a" => {
                asm.push_str(&"SBB    D\n");
                buffer = buffer[2..6].to_owned();
            },            "9b" => {
                asm.push_str(&"SBB    E\n");
                buffer = buffer[2..6].to_owned();
            },            "9c" => {
                asm.push_str(&"SBB    H\n");
                buffer = buffer[2..6].to_owned();
            },            "9d" => {
                asm.push_str(&"SBB    L\n");
                buffer = buffer[2..6].to_owned();
            },            "9e" => {
                asm.push_str(&"SBB    M\n");
                buffer = buffer[2..6].to_owned();
            },            "9f" => {
                asm.push_str(&"SBB    A\n");
                buffer = buffer[2..6].to_owned();
            },            "a0" => {
                asm.push_str(&"ANA    B\n");
                buffer = buffer[2..6].to_owned();
            },            "a1" => {
                asm.push_str(&"ANA    C\n");
                buffer = buffer[2..6].to_owned();
            },            "a2" => {
                asm.push_str(&"ANA    D\n");
                buffer = buffer[2..6].to_owned();
            },            "a3" => {
                asm.push_str(&"ANA    E\n");
                buffer = buffer[2..6].to_owned();
            },            "a4" => {
                asm.push_str(&"ANA    H\n");
                buffer = buffer[2..6].to_owned();
            },            "a5" => {
                asm.push_str(&"ANA    L\n");
                buffer = buffer[2..6].to_owned();
            },            "a6" => {
                asm.push_str(&"ANA    M\n");
                buffer = buffer[2..6].to_owned();
            },            "a7" => {
                asm.push_str(&"ANA    A\n");
                buffer = buffer[2..6].to_owned();
            },            "a8" => {
                asm.push_str(&"XRA    B\n");
                buffer = buffer[2..6].to_owned();
            },            "a9" => {
                asm.push_str(&"XRA    C\n");
                buffer = buffer[2..6].to_owned();
            },            "aa" => {
                asm.push_str(&"XRA    D\n");
                buffer = buffer[2..6].to_owned();
            },            "ab" => {
                asm.push_str(&"XRA    E\n");
                buffer = buffer[2..6].to_owned();
            },            "ac" => {
                asm.push_str(&"XRA    H\n");
                buffer = buffer[2..6].to_owned();
            },            "ad" => {
                asm.push_str(&"XRA    L\n");
                buffer = buffer[2..6].to_owned();
            },            "ae" => {
                asm.push_str(&"XRA    M\n");
                buffer = buffer[2..6].to_owned();
            },            "af" => {
                asm.push_str(&"XRA    A\n");
                buffer = buffer[2..6].to_owned();
            },            "b0" => {
                asm.push_str(&"ORA    B\n");
                buffer = buffer[2..6].to_owned();
            },            "b1" => {
                asm.push_str(&"ORA    C\n");
                buffer = buffer[2..6].to_owned();
            },            "b2" => {
                asm.push_str(&"ORA    D\n");
                buffer = buffer[2..6].to_owned();
            },            "b3" => {
                asm.push_str(&"ORA    E\n");
                buffer = buffer[2..6].to_owned();
            },            "b4" => {
                asm.push_str(&"ORA    H\n");
                buffer = buffer[2..6].to_owned();
            },            "b5" => {
                asm.push_str(&"ORA    L\n");
                buffer = buffer[2..6].to_owned();
            },            "b6" => {
                asm.push_str(&"ORA    M\n");
                buffer = buffer[2..6].to_owned();
            },            "b7" => {
                asm.push_str(&"ORA    A\n");
                buffer = buffer[2..6].to_owned();
            },            "b8" => {
                asm.push_str(&"CMP    B\n");
                buffer = buffer[2..6].to_owned();
            },            "b9" => {
                asm.push_str(&"CMP    C\n");
                buffer = buffer[2..6].to_owned();
            },            "ba" => {
                asm.push_str(&"CMP    D\n");
                buffer = buffer[2..6].to_owned();
            },            "bb" => {
                asm.push_str(&"CMP    E\n");
                buffer = buffer[2..6].to_owned();
            },            "bc" => {
                asm.push_str(&"CMP    H\n");
                buffer = buffer[2..6].to_owned();
            },            "bd" => {
                asm.push_str(&"CMP    L\n");
                buffer = buffer[2..6].to_owned();
            },            "be" => {
                asm.push_str(&"CMP    M\n");
                buffer = buffer[2..6].to_owned();
            },            "bf" => {
                asm.push_str(&"CMP    A\n");
                buffer = buffer[2..6].to_owned();
            },            "c0" => {
                asm.push_str(&"RNZ\n");
                buffer = buffer[2..6].to_owned();
            },            "c1" => {
                asm.push_str(&"POP    B\n");
                buffer = buffer[2..6].to_owned();
            },            "c2" => {
                asm.push_str(&format!("JNZ    ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "c3" => {
                asm.push_str(&format!("JMP    ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "c4" => {
                asm.push_str(&format!("CNZ    ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "c5" => {
                asm.push_str(&"PUSH   B\n");
                buffer = buffer[2..6].to_owned();
            },            "c6" => {
                asm.push_str(&format!("ADI    #${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "c7" => {
                asm.push_str(&"RST    0\n");
                buffer = buffer[2..6].to_owned();
            },            "c8" => {
                asm.push_str(&"RZ\n");
                buffer = buffer[2..6].to_owned();
            },            "c9" => {
                asm.push_str(&"RET\n");
                buffer = buffer[2..6].to_owned();
            },            "ca" => {
                asm.push_str(&format!("JZ     ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "cb" => {
                buffer = buffer[2..6].to_owned();
            },            "cc" => {
                asm.push_str(&format!("CZ     ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "cd" => {
                asm.push_str(&format!("CALL   ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "ce" => {
                asm.push_str(&format!("ACI    #${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "cf" => {
                asm.push_str(&"RST    1\n");
                buffer = buffer[2..6].to_owned();
            },            "d0" => {
                asm.push_str(&"RNC    1\n");
                buffer = buffer[2..6].to_owned();
            },            "d1" => {
                asm.push_str(&"POP    1\n");
                buffer = buffer[2..6].to_owned();
            },            "d2" => {
                asm.push_str(&format!("JNC    ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "d3" => {
                asm.push_str(&format!("OUT    #${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "d4" => {
                asm.push_str(&format!("CNC    ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "d5" => {
                asm.push_str(&"PUSH   D\n");
                buffer = buffer[2..6].to_owned();
            },            "d6" => {
                asm.push_str(&format!("SUI    #${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "d7" => {
                asm.push_str(&"RST    2\n");
                buffer = buffer[2..6].to_owned();
            },            "d8" => {
                asm.push_str(&"RC    1\n");
                buffer = buffer[2..6].to_owned();
            },            "d9" => {
                buffer = buffer[2..6].to_owned();
            },            "da" => {
                asm.push_str(&format!("JC     ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "db" => {
                asm.push_str(&format!("IN     #${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "dc" => {
                asm.push_str(&format!("CC     ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "dd" => {
                buffer = buffer[2..6].to_owned();
            },            "de" => {
                asm.push_str(&format!("SBI    #${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "df" => {
                asm.push_str(&"RST    3\n");
                buffer = buffer[2..6].to_owned();
            },            "e0" => {
                asm.push_str(&"RPO\n");
                buffer = buffer[2..6].to_owned();
            },            "e1" => {
                asm.push_str(&"POP    H\n");
                buffer = buffer[2..6].to_owned();
            },            "e2" => {
                asm.push_str(&format!("JPO    ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "e3" => {
                asm.push_str(&"XHTL\n");
                buffer = buffer[2..6].to_owned();
            },            "e4" => {
                asm.push_str(&format!("CPO    ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "e5" => {
                asm.push_str(&"PUSH   H\n");
                buffer = buffer[2..6].to_owned();
            },            "e6" => {
                asm.push_str(&format!("ANI    #${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "e7" => {
                asm.push_str(&"RST    4\n");
                buffer = buffer[2..6].to_owned();
            },            "e8" => {
                asm.push_str(&"RPE\n");
                buffer = buffer[2..6].to_owned();
            },            "e9" => {
                asm.push_str(&"PCHL\n");
                buffer = buffer[2..6].to_owned();
            },            "ea" => {
                asm.push_str(&format!("JPE    ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "eb" => {
                asm.push_str(&"XCHG\n");
                buffer = buffer[2..6].to_owned();
            },            "ec" => {
                asm.push_str(&format!("CPE   ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "ed" => {
                buffer = buffer[2..6].to_owned();
            },            "ee" => {
                asm.push_str(&format!("XRI    #${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "ef" => {
                asm.push_str(&"RST    5\n");
                buffer = buffer[2..6].to_owned();
            },            "f0" => {
                asm.push_str(&"RP\n");
                buffer = buffer[2..6].to_owned();
            },            "f1" => {
                asm.push_str(&"POP    PSW\n");
                buffer = buffer[2..6].to_owned();
            },            "f2" => {
                asm.push_str(&format!("JP     ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "f3" => {
                asm.push_str(&"DI\n");
                buffer = buffer[2..6].to_owned();
            },            "f4" => {
                asm.push_str(&format!("CP   ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "f5" => {
                asm.push_str(&"PUSH   PSW\n");
                buffer = buffer[2..6].to_owned();
            },            "f6" => {
                asm.push_str(&format!("ORI    #${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "f7" => {
                asm.push_str(&"RST    6\n");
                buffer = buffer[2..6].to_owned();
            },            "f8" => {
                asm.push_str(&"RM\n");
                buffer = buffer[2..6].to_owned();
            },            "f9" => {
                asm.push_str(&"SPHL\n");
                buffer = buffer[2..6].to_owned();
            },            "fa" => {
                asm.push_str(&format!("JM     ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "fb" => {
                asm.push_str(&"EI\n");
                buffer = buffer[2..6].to_owned();
            },            "fc" => {
                asm.push_str(&format!("CM     ${}{}\n", &buffer[2..4], &buffer[4..6]));
                buffer = "".to_owned();
            },            "fd" => {
                buffer = buffer[2..6].to_owned();
            },            "fe" => {
                asm.push_str(&format!("CPI    #${}\n", &buffer[2..4]));
                buffer = buffer[4..6].to_owned();
            },            "ff" => {
                asm.push_str(&"RST    7\n");
                buffer = buffer[2..6].to_owned();
            },
            _ => {
                println!("FATAL ERROR: unexpected byte: {}", &buffer[0..2]);
                return "".to_owned();
            },
        }
    }
    return asm;
}


// Takes the output of hexdump <8080 binary> as input,
// then disassembles it, and writes that output to disassembe.rs
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Improper usage. Please pass the name of the file to disassemble as an argument.");
        return;
    }

    let dumped_hex_as_string = fs::read_to_string(&args[1]).expect("FATAL ERROR: file not readable");
    let dumped_hex_as_byte_slice = dumped_hex_as_string.as_bytes();
    let asm = disassemble(dumped_hex_as_byte_slice);

    fs::write("disassembled.s", &asm).expect("FATAL ERROR: file not writable");
    println!("Generated assembly is in file: disassembled.s");
}
