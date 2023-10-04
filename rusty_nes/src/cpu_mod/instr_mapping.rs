use super::instruction::*;


pub struct InstructionMapping;
 impl InstructionMapping {
    pub fn opcode_to_instruction(opcode: u8) -> Instruction {
        match opcode {
            0 => Instruction::new("BRK", Box::new(BRK), Box::new(IMM), AddrMode::IMM, 7),
            1 => Instruction::new("ORA", Box::new(ORA), Box::new(IZX), AddrMode::IZX, 6),
            2 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            3 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            4 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 3),
            5 => Instruction::new("ORA", Box::new(ORA), Box::new(ZP0), AddrMode::ZP0, 3),
            6 => Instruction::new("ASL", Box::new(ASL), Box::new(ZP0), AddrMode::ZP0, 5),
            7 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            8 => Instruction::new("PHP", Box::new(PHP), Box::new(IMP), AddrMode::IMP, 3),
            9 => Instruction::new("ORA", Box::new(ORA), Box::new(IMM), AddrMode::IMM, 2),
            10 => Instruction::new("ASL", Box::new(ASL), Box::new(IMP), AddrMode::IMP, 2),
            11 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            12 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            13 => Instruction::new("ORA", Box::new(ORA), Box::new(ABS), AddrMode::ABS, 4),
            14 => Instruction::new("ASL", Box::new(ASL), Box::new(ABS), AddrMode::ABS, 6),
            15 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            16 => Instruction::new("BPL", Box::new(BPL), Box::new(REL), AddrMode::REL, 2),
            17 => Instruction::new("ORA", Box::new(ORA), Box::new(IZY), AddrMode::IZY, 5),
            18 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            19 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            20 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            21 => Instruction::new("ORA", Box::new(ORA), Box::new(ZPX), AddrMode::ZPX, 4),
            22 => Instruction::new("ASL", Box::new(ASL), Box::new(ZPX), AddrMode::ZPX, 6),
            23 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            24 => Instruction::new("CLC", Box::new(CLC), Box::new(IMP), AddrMode::IMP, 2),
            25 => Instruction::new("ORA", Box::new(ORA), Box::new(ABY), AddrMode::ABY, 4),
            26 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            27 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            28 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            29 => Instruction::new("ORA", Box::new(ORA), Box::new(ABX), AddrMode::ABX, 4),
            30 => Instruction::new("ASL", Box::new(ASL), Box::new(ABX), AddrMode::ABX, 7),
            31 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            32 => Instruction::new("JSR", Box::new(JSR), Box::new(ABS), AddrMode::ABS, 6),
            33 => Instruction::new("AND", Box::new(AND), Box::new(IZX), AddrMode::IZX, 6),
            34 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            35 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            36 => Instruction::new("BIT", Box::new(BIT), Box::new(ZP0), AddrMode::ZP0, 3),
            37 => Instruction::new("AND", Box::new(AND), Box::new(ZP0), AddrMode::ZP0, 3),
            38 => Instruction::new("ROL", Box::new(ROL), Box::new(ZP0), AddrMode::ZP0, 5),
            39 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            40 => Instruction::new("PLP", Box::new(PLP), Box::new(IMP), AddrMode::IMP, 4),
            41 => Instruction::new("AND", Box::new(AND), Box::new(IMM), AddrMode::IMM, 2),
            42 => Instruction::new("ROL", Box::new(ROL), Box::new(IMP), AddrMode::IMP, 2),
            43 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            44 => Instruction::new("BIT", Box::new(BIT), Box::new(ABS), AddrMode::ABS, 4),
            45 => Instruction::new("AND", Box::new(AND), Box::new(ABS), AddrMode::ABS, 4),
            46 => Instruction::new("ROL", Box::new(ROL), Box::new(ABS), AddrMode::ABS, 6),
            47 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            48 => Instruction::new("BMI", Box::new(BMI), Box::new(REL), AddrMode::REL, 2),
            49 => Instruction::new("AND", Box::new(AND), Box::new(IZY), AddrMode::IZY, 5),
            50 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            51 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            52 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            53 => Instruction::new("AND", Box::new(AND), Box::new(ZPX), AddrMode::ZPX, 4),
            54 => Instruction::new("ROL", Box::new(ROL), Box::new(ZPX), AddrMode::ZPX, 6),
            55 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            56 => Instruction::new("SEC", Box::new(SEC), Box::new(IMP), AddrMode::IMP, 2),
            57 => Instruction::new("AND", Box::new(AND), Box::new(ABY), AddrMode::ABY, 4),
            58 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            59 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            60 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            61 => Instruction::new("AND", Box::new(AND), Box::new(ABX), AddrMode::ABX, 4),
            62 => Instruction::new("ROL", Box::new(ROL), Box::new(ABX), AddrMode::ABX, 7),
            63 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            64 => Instruction::new("RTI", Box::new(RTI), Box::new(IMP), AddrMode::IMP, 6),
            65 => Instruction::new("EOR", Box::new(EOR), Box::new(IZX), AddrMode::IZX, 6),
            66 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            67 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            68 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 3),
            69 => Instruction::new("EOR", Box::new(EOR), Box::new(ZP0), AddrMode::ZP0, 3),
            70 => Instruction::new("LSR", Box::new(LSR), Box::new(ZP0), AddrMode::ZP0, 5),
            71 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            72 => Instruction::new("PHA", Box::new(PHA), Box::new(IMP), AddrMode::IMP, 3),
            73 => Instruction::new("EOR", Box::new(EOR), Box::new(IMM), AddrMode::IMM, 2),
            74 => Instruction::new("LSR", Box::new(LSR), Box::new(IMP), AddrMode::IMP, 2),
            75 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            76 => Instruction::new("JMP", Box::new(JMP), Box::new(ABS), AddrMode::ABS, 3),
            77 => Instruction::new("EOR", Box::new(EOR), Box::new(ABS), AddrMode::ABS, 4),
            78 => Instruction::new("LSR", Box::new(LSR), Box::new(ABS), AddrMode::ABS, 6),
            79 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            80 => Instruction::new("BVC", Box::new(BVC), Box::new(REL), AddrMode::REL, 2),
            81 => Instruction::new("EOR", Box::new(EOR), Box::new(IZY), AddrMode::IZY, 5),
            82 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            83 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            84 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            85 => Instruction::new("EOR", Box::new(EOR), Box::new(ZPX), AddrMode::ZPX, 4),
            86 => Instruction::new("LSR", Box::new(LSR), Box::new(ZPX), AddrMode::ZPX, 6),
            87 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            88 => Instruction::new("CLI", Box::new(CLI), Box::new(IMP), AddrMode::IMP, 2),
            89 => Instruction::new("EOR", Box::new(EOR), Box::new(ABY), AddrMode::ABY, 4),
            90 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            91 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            92 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            93 => Instruction::new("EOR", Box::new(EOR), Box::new(ABX), AddrMode::ABX, 4),
            94 => Instruction::new("LSR", Box::new(LSR), Box::new(ABX), AddrMode::ABX, 7),
            95 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            96 => Instruction::new("RTS", Box::new(RTS), Box::new(IMP), AddrMode::IMP, 6),
            97 => Instruction::new("ADC", Box::new(ADC), Box::new(IZX), AddrMode::IZX, 6),
            98 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            99 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            100 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 3),
            101 => Instruction::new("ADC", Box::new(ADC), Box::new(ZP0), AddrMode::ZP0, 3),
            102 => Instruction::new("ROR", Box::new(ROR), Box::new(ZP0), AddrMode::ZP0, 5),
            103 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            104 => Instruction::new("PLA", Box::new(PLA), Box::new(IMP), AddrMode::IMP, 4),
            105 => Instruction::new("ADC", Box::new(ADC), Box::new(IMM), AddrMode::IMM, 2),
            106 => Instruction::new("ROR", Box::new(ROR), Box::new(IMP), AddrMode::IMP, 2),
            107 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            108 => Instruction::new("JMP", Box::new(JMP), Box::new(IND), AddrMode::IND, 5),
            109 => Instruction::new("ADC", Box::new(ADC), Box::new(ABS), AddrMode::ABS, 4),
            110 => Instruction::new("ROR", Box::new(ROR), Box::new(ABS), AddrMode::ABS, 6),
            111 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            112 => Instruction::new("BVS", Box::new(BVS), Box::new(REL), AddrMode::REL, 2),
            113 => Instruction::new("ADC", Box::new(ADC), Box::new(IZY), AddrMode::IZY, 5),
            114 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            115 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            116 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            117 => Instruction::new("ADC", Box::new(ADC), Box::new(ZPX), AddrMode::ZPX, 4),
            118 => Instruction::new("ROR", Box::new(ROR), Box::new(ZPX), AddrMode::ZPX, 6),
            119 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            120 => Instruction::new("SEI", Box::new(SEI), Box::new(IMP), AddrMode::IMP, 2),
            121 => Instruction::new("ADC", Box::new(ADC), Box::new(ABY), AddrMode::ABY, 4),
            122 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            123 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            124 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            125 => Instruction::new("ADC", Box::new(ADC), Box::new(ABX), AddrMode::ABX, 4),
            126 => Instruction::new("ROR", Box::new(ROR), Box::new(ABX), AddrMode::ABX, 7),
            127 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            128 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            129 => Instruction::new("STA", Box::new(STA), Box::new(IZX), AddrMode::IZX, 6),
            130 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            131 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            132 => Instruction::new("STY", Box::new(STY), Box::new(ZP0), AddrMode::ZP0, 3),
            133 => Instruction::new("STA", Box::new(STA), Box::new(ZP0), AddrMode::ZP0, 3),
            134 => Instruction::new("STX", Box::new(STX), Box::new(ZP0), AddrMode::ZP0, 3),
            135 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 3),
            136 => Instruction::new("DEY", Box::new(DEY), Box::new(IMP), AddrMode::IMP, 2),
            137 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            138 => Instruction::new("TXA", Box::new(TXA), Box::new(IMP), AddrMode::IMP, 2),
            139 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            140 => Instruction::new("STY", Box::new(STY), Box::new(ABS), AddrMode::ABS, 4),
            141 => Instruction::new("STA", Box::new(STA), Box::new(ABS), AddrMode::ABS, 4),
            142 => Instruction::new("STX", Box::new(STX), Box::new(ABS), AddrMode::ABS, 4),
            143 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            144 => Instruction::new("BCC", Box::new(BCC), Box::new(REL), AddrMode::REL, 2),
            145 => Instruction::new("STA", Box::new(STA), Box::new(IZY), AddrMode::IZY, 6),
            146 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            147 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            148 => Instruction::new("STY", Box::new(STY), Box::new(ZPX), AddrMode::ZPX, 4),
            149 => Instruction::new("STA", Box::new(STA), Box::new(ZPX), AddrMode::ZPX, 4),
            150 => Instruction::new("STX", Box::new(STX), Box::new(ZPY), AddrMode::ZPY, 4),
            151 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            152 => Instruction::new("TYA", Box::new(TYA), Box::new(IMP), AddrMode::IMP, 2),
            153 => Instruction::new("STA", Box::new(STA), Box::new(ABY), AddrMode::ABY, 5),
            154 => Instruction::new("TXS", Box::new(TXS), Box::new(IMP), AddrMode::IMP, 2),
            155 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            156 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 5),
            157 => Instruction::new("STA", Box::new(STA), Box::new(ABX), AddrMode::ABX, 5),
            158 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            159 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            160 => Instruction::new("LDY", Box::new(LDY), Box::new(IMM), AddrMode::IMM, 2),
            161 => Instruction::new("LDA", Box::new(LDA), Box::new(IZX), AddrMode::IZX, 6),
            162 => Instruction::new("LDX", Box::new(LDX), Box::new(IMM), AddrMode::IMM, 2),
            163 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            164 => Instruction::new("LDY", Box::new(LDY), Box::new(ZP0), AddrMode::ZP0, 3),
            165 => Instruction::new("LDA", Box::new(LDA), Box::new(ZP0), AddrMode::ZP0, 3),
            166 => Instruction::new("LDX", Box::new(LDX), Box::new(ZP0), AddrMode::ZP0, 3),
            167 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 3),
            168 => Instruction::new("TAY", Box::new(TAY), Box::new(IMP), AddrMode::IMP, 2),
            169 => Instruction::new("LDA", Box::new(LDA), Box::new(IMM), AddrMode::IMM, 2),
            170 => Instruction::new("TAX", Box::new(TAX), Box::new(IMP), AddrMode::IMP, 2),
            171 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            172 => Instruction::new("LDY", Box::new(LDY), Box::new(ABS), AddrMode::ABS, 4),
            173 => Instruction::new("LDA", Box::new(LDA), Box::new(ABS), AddrMode::ABS, 4),
            174 => Instruction::new("LDX", Box::new(LDX), Box::new(ABS), AddrMode::ABS, 4),
            175 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            176 => Instruction::new("BCS", Box::new(BCS), Box::new(REL), AddrMode::REL, 2),
            177 => Instruction::new("LDA", Box::new(LDA), Box::new(IZY), AddrMode::IZY, 5),
            178 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            179 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            180 => Instruction::new("LDY", Box::new(LDY), Box::new(ZPX), AddrMode::ZPX, 4),
            181 => Instruction::new("LDA", Box::new(LDA), Box::new(ZPX), AddrMode::ZPX, 4),
            182 => Instruction::new("LDX", Box::new(LDX), Box::new(ZPY), AddrMode::ZPY, 4),
            183 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            184 => Instruction::new("CLV", Box::new(CLV), Box::new(IMP), AddrMode::IMP, 2),
            185 => Instruction::new("LDA", Box::new(LDA), Box::new(ABY), AddrMode::ABY, 4),
            186 => Instruction::new("TSX", Box::new(TSX), Box::new(IMP), AddrMode::IMP, 2),
            187 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            188 => Instruction::new("LDY", Box::new(LDY), Box::new(ABX), AddrMode::ABX, 4),
            189 => Instruction::new("LDA", Box::new(LDA), Box::new(ABX), AddrMode::ABX, 4),
            190 => Instruction::new("LDX", Box::new(LDX), Box::new(ABY), AddrMode::ABY, 4),
            191 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            192 => Instruction::new("CPY", Box::new(CPY), Box::new(IMM), AddrMode::IMM, 2),
            193 => Instruction::new("CMP", Box::new(CMP), Box::new(IZX), AddrMode::IZX, 6),
            194 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            195 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            196 => Instruction::new("CPY", Box::new(CPY), Box::new(ZP0), AddrMode::ZP0, 3),
            197 => Instruction::new("CMP", Box::new(CMP), Box::new(ZP0), AddrMode::ZP0, 3),
            198 => Instruction::new("DEC", Box::new(DEC), Box::new(ZP0), AddrMode::ZP0, 5),
            199 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            200 => Instruction::new("INY", Box::new(INY), Box::new(IMP), AddrMode::IMP, 2),
            201 => Instruction::new("CMP", Box::new(CMP), Box::new(IMM), AddrMode::IMM, 2),
            202 => Instruction::new("DEX", Box::new(DEX), Box::new(IMP), AddrMode::IMP, 2),
            203 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            204 => Instruction::new("CPY", Box::new(CPY), Box::new(ABS), AddrMode::ABS, 4),
            205 => Instruction::new("CMP", Box::new(CMP), Box::new(ABS), AddrMode::ABS, 4),
            206 => Instruction::new("DEC", Box::new(DEC), Box::new(ABS), AddrMode::ABS, 6),
            207 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            208 => Instruction::new("BNE", Box::new(BNE), Box::new(REL), AddrMode::REL, 2),
            209 => Instruction::new("CMP", Box::new(CMP), Box::new(IZY), AddrMode::IZY, 5),
            210 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            211 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            212 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            213 => Instruction::new("CMP", Box::new(CMP), Box::new(ZPX), AddrMode::ZPX, 4),
            214 => Instruction::new("DEC", Box::new(DEC), Box::new(ZPX), AddrMode::ZPX, 6),
            215 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            216 => Instruction::new("CLD", Box::new(CLD), Box::new(IMP), AddrMode::IMP, 2),
            217 => Instruction::new("CMP", Box::new(CMP), Box::new(ABY), AddrMode::ABY, 4),
            218 => Instruction::new("NOP", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            219 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            220 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            221 => Instruction::new("CMP", Box::new(CMP), Box::new(ABX), AddrMode::ABX, 4),
            222 => Instruction::new("DEC", Box::new(DEC), Box::new(ABX), AddrMode::ABX, 7),
            223 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            224 => Instruction::new("CPX", Box::new(CPX), Box::new(IMM), AddrMode::IMM, 2),
            225 => Instruction::new("SBC", Box::new(SBC), Box::new(IZX), AddrMode::IZX, 6),
            226 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            227 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            228 => Instruction::new("CPX", Box::new(CPX), Box::new(ZP0), AddrMode::ZP0, 3),
            229 => Instruction::new("SBC", Box::new(SBC), Box::new(ZP0), AddrMode::ZP0, 3),
            230 => Instruction::new("INC", Box::new(INC), Box::new(ZP0), AddrMode::ZP0, 5),
            231 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            232 => Instruction::new("INX", Box::new(INX), Box::new(IMP), AddrMode::IMP, 2),
            233 => Instruction::new("SBC", Box::new(SBC), Box::new(IMM), AddrMode::IMM, 2),
            234 => Instruction::new("NOP", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            235 => Instruction::new("???", Box::new(SBC), Box::new(IMP), AddrMode::IMP, 2),
            236 => Instruction::new("CPX", Box::new(CPX), Box::new(ABS), AddrMode::ABS, 4),
            237 => Instruction::new("SBC", Box::new(SBC), Box::new(ABS), AddrMode::ABS, 4),
            238 => Instruction::new("INC", Box::new(INC), Box::new(ABS), AddrMode::ABS, 6),
            239 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            240 => Instruction::new("BEQ", Box::new(BEQ), Box::new(REL), AddrMode::REL, 2),
            241 => Instruction::new("SBC", Box::new(SBC), Box::new(IZY), AddrMode::IZY, 5),
            242 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            243 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            244 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            245 => Instruction::new("SBC", Box::new(SBC), Box::new(ZPX), AddrMode::ZPX, 4),
            246 => Instruction::new("INC", Box::new(INC), Box::new(ZPX), AddrMode::ZPX, 6),
            247 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            248 => Instruction::new("SED", Box::new(SED), Box::new(IMP), AddrMode::IMP, 2),
            249 => Instruction::new("SBC", Box::new(SBC), Box::new(ABY), AddrMode::ABY, 4),
            250 => Instruction::new("NOP", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            251 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            252 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            253 => Instruction::new("SBC", Box::new(SBC), Box::new(ABX), AddrMode::ABX, 4),
            254 => Instruction::new("INC", Box::new(INC), Box::new(ABX), AddrMode::ABX, 7),
            255 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
        }
    }
}