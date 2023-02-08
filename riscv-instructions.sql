
PRAGMA foreign_keys=ON;
BEGIN TRANSACTION;
CREATE TABLE traits_priority ( -- If different traits may work it's important to pick the less complex one.
    priority INTEGER PRIMARY KEY, -- But of complexity is the same we still need some stability.
    name TEXT NOT NULL UNIQUE
);
INSERT INTO traits_priority VALUES(0,'source_assembler_operand');
INSERT INTO traits_priority VALUES(1,'destination_assembler_operand');
INSERT INTO traits_priority VALUES(2,'csr_assembler_operand');
INSERT INTO traits_priority VALUES(3,'prefetch_assembler_operand');
INSERT INTO traits_priority VALUES(4,'shift_rv32_assembler_operand');
INSERT INTO traits_priority VALUES(5,'shift_rv64_assembler_operand');
INSERT INTO traits_priority VALUES(6,'zero_offset_base_assembler_operand');

CREATE TABLE traits_information ( -- Different traits support different assembler operands and we need to pick the right one.
    name TEXT NOT NULL,
    allowed_operand TEXT NOT NULL,
    PRIMARY KEY(allowed_operand, name)
    FOREIGN KEY(name) REFERENCES traits_priority(name)
);
INSERT INTO traits_information VALUES('csr_assembler_operand','gpr');
INSERT INTO traits_information VALUES('csr_assembler_operand','c:imm');
INSERT INTO traits_information VALUES('destination_assembler_operand','gpr');
INSERT INTO traits_information VALUES('destination_assembler_operand','fpr');
INSERT INTO traits_information VALUES('destination_assembler_operand','s:imm');
INSERT INTO traits_information VALUES('destination_assembler_operand','s:imm(gpr)');
INSERT INTO traits_information VALUES('prefetch_assembler_operand','p:imm(gpr)');
INSERT INTO traits_information VALUES('shift_rv32_assembler_operand','>:imm');
INSERT INTO traits_information VALUES('shift_rv32_assembler_operand','gpr');
INSERT INTO traits_information VALUES('shift_rv64_assembler_operand','<:imm');
INSERT INTO traits_information VALUES('shift_rv64_assembler_operand','gpr');
INSERT INTO traits_information VALUES('source_assembler_operand','b:imm');
INSERT INTO traits_information VALUES('source_assembler_operand','csr');
INSERT INTO traits_information VALUES('source_assembler_operand','fpr');
INSERT INTO traits_information VALUES('source_assembler_operand','i:imm');
INSERT INTO traits_information VALUES('source_assembler_operand','i:imm(gpr)');
INSERT INTO traits_information VALUES('source_assembler_operand','j:imm');
INSERT INTO traits_information VALUES('source_assembler_operand','gpr');
INSERT INTO traits_information VALUES('source_assembler_operand','fence');
INSERT INTO traits_information VALUES('source_assembler_operand','rm');
INSERT INTO traits_information VALUES('source_assembler_operand','u:imm');
INSERT INTO traits_information VALUES('zero_offset_base_assembler_operand','0(gpr)');

CREATE TABLE operand ( -- Note that in RISC-V most instructions have only one set of permissible operands.
    name TEXT NOT NULL,         -- There are some aliases (e.g. "add" name can be used with immediate), but even these.
    operand_source TEXT CHECK ( -- don't requre complex selection of traits, like in x86 case
        operand_source IN ( -- Operands can be encoded in a few predefined places in machine word.
            '<:imm',     -- Shift immediate (0..31 for RV32, 0..63 for RV64).
            '>:imm',     -- Short shift immediate (0..31 for RV64).
            'b:imm',     -- Branch immediates (B-immediate).
            'c:imm',     -- CSR immediate (in place of rs1).
            'csr',       -- CSR register.
            'fencep',    -- Fence predecessor operand.
            'fences',    -- Fence successor operand.
            'i:imm',     -- Immediate (I-immediate)
            'j:imm',     -- Jump immediate (J-immediate).
            'rd',        -- Register destination operand.
            'rs1',       -- Register first source operand.
            'rs2',       -- Register second source operand.
            'rs3',       -- Register third source operand (FMA).
            'rm',        -- Immediate rounding mode.
            's:imm',     -- Destination immediate (S-immediate).
            'u:imm',     -- Upper immediate (U-immediate).
            '0(rs1)',    -- Source/Destinatin with zero offset.
            'i:imm(rs1)',  -- Address with I-immediate offset.
            'p:imm(rs1)', -- Address with P-immediate offset (prefetch instructions).
            's:imm(rs1)'  -- Address with S-immediate offset.
        )
    ) NOT NULL,
    parameter_type TEXT CHECK (
        parameter_type IN (
            '<:imm',      -- 5/6 bit shift immediate.
            '>:imm',      -- 5 bit shift immediate.
            '0(gpr)',     -- Pure address immediate.
            'b:imm',      -- Branch immediates (B-immediate).
            'c:imm',      -- CSR immediate (in place of rs1).
            'csr',        -- CSR register (in place of imm).
            'fence',      -- Fence operand.
            'fpr',        -- Floating-point register.
            'gpr',        -- General-purpose register.
            'i:imm',        -- Immediate.
            'i:imm(gpr)',   -- Immediate+gpr = address.
            'j:imm',      -- Jump immediate (J-immediate).
            'p:imm(gpr)', -- Prefetch immediate+gpr address.
            'rm',         -- RM field immediate.
            's:imm',      -- Destination immediate.
            's:imm(gpr)', -- Destination immediate+gpr address.
            'u:imm'       -- Upper immediate (U-immediate).
        )
    )  NOT NULL,
    PRIMARY KEY(name, operand_source, parameter_type)
);
INSERT INTO operand VALUES('<:imm','<:imm','<:imm');
INSERT INTO operand VALUES('>:imm','>:imm','>:imm');
INSERT INTO operand VALUES('csr','csr','csr');
INSERT INTO operand VALUES('fd','rd','fpr');
INSERT INTO operand VALUES('fs1','rs1','fpr');
INSERT INTO operand VALUES('fs2','rs2','fpr');
INSERT INTO operand VALUES('fs3','rs3','fpr');
INSERT INTO operand VALUES('rd','rd','gpr');
INSERT INTO operand VALUES('rs1','rs1','gpr');
INSERT INTO operand VALUES('rs2','rs2','gpr');
INSERT INTO operand VALUES('fencep','fencep','fence');
INSERT INTO operand VALUES('fences','fences','fence');
INSERT INTO operand VALUES('rm','rm','rm');
INSERT INTO operand VALUES('b:imm','b:imm','b:imm');
INSERT INTO operand VALUES('c:imm','c:imm','c:imm');
INSERT INTO operand VALUES('i:imm','i:imm','i:imm');
INSERT INTO operand VALUES('j:imm','j:imm','j:imm');
INSERT INTO operand VALUES('s:imm','s:imm','s:imm');
INSERT INTO operand VALUES('u:imm','u:imm','u:imm');
INSERT INTO operand VALUES('0(rs1)','0(rs1)','0(gpr)');
INSERT INTO operand VALUES('i:imm(rs1)','i:imm(rs1)','i:imm(gpr)');
INSERT INTO operand VALUES('p:imm(rs1)','p:imm(rs1)','p:imm(gpr)');
INSERT INTO operand VALUES('s:imm(rs1)','s:imm(rs1)','s:imm(gpr)');

CREATE TABLE operands ( -- Separated from instructions table mostly to catch mistakes.
                        -- Many combinations are forbidden and it's easier to whitelist them than write aleborate CHECKs.
    short_name TEXT PRIMARY KEY,
    operand0 TEXT CHECK (operand0 IS NULL OR operand0 IN ('fencep', 'p:imm(rs1)', 'fd', 'fs2', 'rd', 'rs1', 'rs2')) NULL DEFAULT NULL,
    operand1 TEXT CHECK (operand1 IS NULL OR operand1 IN ('0(rs1)', 'csr', 'fences', 'fs1', 'i:imm(rs1)', 'j:imm', 'rs1', 'rs2', 's:imm(rs1)', 'u:imm')) NULL DEFAULT NULL,
    operand2 TEXT CHECK (operand2 IS NULL OR operand2 IN ('<:imm', '>:imm', 'b:imm', 'c:imm', 'fs2', 'i:imm', 'rm', '0(rs1)', 'rs1', 'rs2')) NULL DEFAULT NULL,
    operand3 TEXT CHECK (operand3 IS NULL OR operand3 IN ('fs3', 'rm')) NULL DEFAULT NULL,
    operand4 TEXT CHECK (operand4 IS NULL OR operand4 IN ('rm')) NULL DEFAULT NULL
);
INSERT INTO operands VALUES(NULL,NULL,NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('fd-fs1-fs2','fd','fs1','fs2',NULL,NULL);
INSERT INTO operands VALUES('fd-fs1-fs2-fs3-rm','fd','fs1','fs2','fs3','rm');
INSERT INTO operands VALUES('fd-fs1-fs2-rm','fd','fs1','fs2','rm',NULL);
INSERT INTO operands VALUES('fd-fs1-rm','fd','fs1','rm',NULL,NULL);
INSERT INTO operands VALUES('fd-i:imm(rs1)','fd','i:imm(rs1)',NULL,NULL,NULL);
INSERT INTO operands VALUES('fd-rs1','fd','rs1',NULL,NULL,NULL);
INSERT INTO operands VALUES('fd-rs1-rm','fd','rs1','rm',NULL,NULL);
INSERT INTO operands VALUES('fencep-fences','fencep','fences',NULL,NULL,NULL);
INSERT INTO operands VALUES('fs2-s:imm(rs1)','fs2','s:imm(rs1)',NULL,NULL,NULL);
INSERT INTO operands VALUES('p:imm(rs1)','p:imm(rs1)',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('rd-0(rs1)','rd','0(rs1)',NULL,NULL,NULL);
INSERT INTO operands VALUES('rd-csr-c:imm','rd','csr','c:imm',NULL,NULL);
INSERT INTO operands VALUES('rd-csr-rs1','rd','csr','rs1',NULL,NULL);
INSERT INTO operands VALUES('rd-fs1','rd','fs1',NULL,NULL,NULL);
INSERT INTO operands VALUES('rd-fs1-fs2','rd','fs1','fs2',NULL,NULL);
INSERT INTO operands VALUES('rd-fs1-rm','rd','fs1','rm',NULL,NULL);
INSERT INTO operands VALUES('rd-i:imm(rs1)','rd','i:imm(rs1)',NULL,NULL,NULL);
INSERT INTO operands VALUES('rd-j:imm','rd','j:imm',NULL,NULL,NULL);
INSERT INTO operands VALUES('rd-rs1-<:imm','rd','rs1','<:imm',NULL,NULL);
INSERT INTO operands VALUES('rd-rs1->:imm','rd','rs1','>:imm',NULL,NULL);
INSERT INTO operands VALUES('rd-rs1-i:imm','rd','rs1','i:imm',NULL,NULL);
INSERT INTO operands VALUES('rd-rs1-rs2','rd','rs1','rs2',NULL,NULL);
INSERT INTO operands VALUES('rd-rs2-0(rs1)','rd','rs2','0(rs1)',NULL,NULL);
INSERT INTO operands VALUES('rd-u:imm','rd','u:imm',NULL,NULL,NULL);
INSERT INTO operands VALUES('rs1-rs2-b:imm','rs1','rs2','b:imm',NULL,NULL);
INSERT INTO operands VALUES('rs2-s:imm(rs1)','rs2','s:imm(rs1)',NULL,NULL,NULL);


CREATE TABLE instruction (
    name TEXT NOT NULL, -- Instructrion name.
    operands TEXT CHECK ( -- operands. See “The RISC-V Instruction Set Manual.”
        operands IS NULL OR
        operands IN (
            'fd-fs1-fs2', -- e.g. fsgnj.s f1, f2, f3 
            'fd-fs1-fs2-fs3-rm', -- e.g. fmadd.s f1, f2, f3, f4, rne 
            'fd-fs1-fs2-rm', -- e.g. fsub.s f1, f2, f3, rne
            'fd-fs1-rm', -- e.g fsqrt.s f1, f2, rne
            'fd-i:imm(rs1)', -- e.g. flw f1, 1234(x2) 
            'fd-rs1', -- e.g. fmv.x.d f1, x1
            'fd-rs1-rm', -- e.g. fmv.x.w f1, x2, rne
            'fencep-fences', -- e.g. fence iorw, iorw
            'fs2-s:imm(rs1)', -- e.g. fsw f1, 1234(x2)
            'p:imm(rs1)',    -- e.g. prefetch 32(x1)
            'rd-0(rs1)',     -- e.g. lr.w x1, 0(x3)
            'rd-csr-c:imm',  -- e.g. csrrwi x1, fcsr, 0
            'rd-csr-rs1',    -- e.g. csrrw x1, fcsr, x0
            'rd-fs1', -- e.g. fclass.s x1, f2
            'rd-fs1-fs2', -- e.g. feq.s x1, f2, f3
            'rd-fs1-rm', -- e.g. fcvt.w.s x1, f2, rne
            'rd-i:imm(rs1)',   -- e.g. jalr x1, 2(x3)
            'rd-j:imm',      -- e.g. jal 1234
            'rd-rs1-<:imm',  -- e.g. slliw x1, x2, 45
            'rd-rs1->:imm',  -- e.g. slli x1, x2, 45
            'rd-rs1-i:imm',    -- e.g. jalr x1, x2, 3
            'rd-rs1-rs2',    -- e.g. add x1, x2, x3
            'rd-rs2-0(rs1)', -- e.g. sc.w x1, x2, 0(x3)
            'rd-u:imm',      -- e.g. lui x1, 5054464
            'rs1-rs2-b:imm', -- e.g. beq x1, x2, 1234
            'rs2-s:imm(rs1)' -- e.g. sw x1, 1234(x2)
        )
    ) NULL DEFAULT NULL,
    assembler_kind TEXT CHECK( -- Certain instructions can only be used in a specific assembler mode.
        assembler_kind IS NULL OR
        assembler_kind IN (
            'rv32',
            'rv64'
        )
    ) NULL DEFAULT NULL,
    opcode INTEGER NOT NULL,
    opcode_mask INTEGER NOT NULL,
    extension TEXT CHECK (  -- RISC V extensions.
        extension IN (
            'A',           -- Atomic instructions.
            'B',           -- Bit-Manipulation extension (tentatively reserved).
            'C',           -- Compressed extension.
            'D',           -- Double-precision floating-point extension.
            'E',           -- RV32E base ISA (tentatively reserved).
            'F',           -- Single-precision floating-point extension.
            'H',           -- Hypervisor extension.
            'I',           -- RV32I/64I/128I base ISA.
            'J',           -- Dynamically Translated Languages extension (tentatively reserved).
            'M',           -- Integer Multiply/Divide extension.
            'N',           -- User-Level Interrupts extension (tentatively reserved).
            'P',           -- Packed-SIMD extension (tentatively reserved).
            'Q',           -- Quad-precision floating-point extension.
            'S',           -- Supervisor mode implemented.
            'U',           -- User mode implemented.
            'V',           -- Vector extension (tentatively reserved).
            'Zam',         -- Misaligned Atomics.
            'Zicbop',      -- Prefetch instructions (mapped to ORI thus always available).
            'Zicsr',       -- Control and Status Register (CSR) Instructions.
            'Zifencei',    -- Instruction-Fetch Fence,
            'Zihintpause', -- Pause instruction (mapped to FENCE with pref=W, succ=0 thus always available).
            'Ztso'         -- Total Store Ordering.
        )
    ) NOT NULL,
    PRIMARY KEY(name, operands, assembler_kind, opcode, opcode_mask),
    FOREIGN KEY(operands) REFERENCES operands(short_name)
);


--INSERT INTO instruction VALUES('jalr','rd-rs1-imm',NULL,0x00000067,0x0000707f,'I');

INSERT INTO instruction VALUES('add','rd-rs1-rs2',NULL,0x00000033,0xfe00707f,'I');
INSERT INTO instruction VALUES('addi','rd-rs1-i:imm',NULL,0x00000013,0x0000707f,'I');
INSERT INTO instruction VALUES('addiw','rd-rs1-i:imm','rv64',0x0000001b,0x0000707f,'I');
INSERT INTO instruction VALUES('addw','rd-rs1-rs2','rv64',0x0000003b,0xfe00707f,'I');
INSERT INTO instruction VALUES('amoadd.d','rd-rs2-0(rs1)','rv64',0x0000302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoadd.d.aq','rd-rs2-0(rs1)','rv64',0x0400302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoadd.d.aqrl','rd-rs2-0(rs1)','rv64',0x0600302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoadd.d.rl','rd-rs2-0(rs1)','rv64',0x0200302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoadd.w','rd-rs2-0(rs1)',NULL,0x0000202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoadd.w.aq','rd-rs2-0(rs1)',NULL,0x0400202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoadd.w.aqrl','rd-rs2-0(rs1)',NULL,0x0600202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoadd.w.rl','rd-rs2-0(rs1)',NULL,0x0200202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoand.d','rd-rs2-0(rs1)','rv64',0x6000302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoand.d.aq','rd-rs2-0(rs1)','rv64',0x6400302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoand.d.aqrl','rd-rs2-0(rs1)','rv64',0x6600302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoand.d.rl','rd-rs2-0(rs1)','rv64',0x6200302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoand.w','rd-rs2-0(rs1)',NULL,0x6000202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoand.w.aq','rd-rs2-0(rs1)',NULL,0x6400202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoand.w.aqrl','rd-rs2-0(rs1)',NULL,0x6600202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoand.w.rl','rd-rs2-0(rs1)',NULL,0x6200202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomax.d','rd-rs2-0(rs1)','rv64',0xa000302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomax.d.aq','rd-rs2-0(rs1)','rv64',0xa400302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomax.d.aqrl','rd-rs2-0(rs1)','rv64',0xa600302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomax.d.rl','rd-rs2-0(rs1)','rv64',0xa200302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomax.w','rd-rs2-0(rs1)',NULL,0xa000202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomax.w.aq','rd-rs2-0(rs1)',NULL,0xa400202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomax.w.aqrl','rd-rs2-0(rs1)',NULL,0xa600202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomax.w.rl','rd-rs2-0(rs1)',NULL,0xa200202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomaxu.d','rd-rs2-0(rs1)','rv64',0xe000302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomaxu.d.aq','rd-rs2-0(rs1)','rv64',0xe400302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomaxu.d.aqrl','rd-rs2-0(rs1)','rv64',0xe600302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomaxu.d.rl','rd-rs2-0(rs1)','rv64',0xe200302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomaxu.w','rd-rs2-0(rs1)',NULL,0xe000202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomaxu.w.aq','rd-rs2-0(rs1)',NULL,0xe400202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomaxu.w.aqrl','rd-rs2-0(rs1)',NULL,0xe600202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomaxu.w.rl','rd-rs2-0(rs1)',NULL,0xe200202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomin.d','rd-rs2-0(rs1)','rv64',0x8000302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomin.d.aq','rd-rs2-0(rs1)','rv64',0x8400302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomin.d.aqrl','rd-rs2-0(rs1)','rv64',0x8600302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomin.d.rl','rd-rs2-0(rs1)','rv64',0x8200302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomin.w','rd-rs2-0(rs1)',NULL,0x8000202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomin.w.aq','rd-rs2-0(rs1)',NULL,0x8400202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomin.w.aqrl','rd-rs2-0(rs1)',NULL,0x8600202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amomin.w.rl','rd-rs2-0(rs1)',NULL,0x8200202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amominu.d','rd-rs2-0(rs1)','rv64',0xc000302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amominu.d.aq','rd-rs2-0(rs1)','rv64',0xc400302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amominu.d.aqrl','rd-rs2-0(rs1)','rv64',0xc600302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amominu.d.rl','rd-rs2-0(rs1)','rv64',0xc200302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amominu.w','rd-rs2-0(rs1)',NULL,0xc000202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amominu.w.aq','rd-rs2-0(rs1)',NULL,0xc400202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amominu.w.aqrl','rd-rs2-0(rs1)',NULL,0xc600202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amominu.w.rl','rd-rs2-0(rs1)',NULL,0xc200202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoor.d','rd-rs2-0(rs1)','rv64',0x4000302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoor.d.aq','rd-rs2-0(rs1)','rv64',0x4400302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoor.d.aqrl','rd-rs2-0(rs1)','rv64',0x4600302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoor.d.rl','rd-rs2-0(rs1)','rv64',0x4200302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoor.w','rd-rs2-0(rs1)',NULL,0x4000202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoor.w.aq','rd-rs2-0(rs1)',NULL,0x4400202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoor.w.aqrl','rd-rs2-0(rs1)',NULL,0x4600202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoor.w.rl','rd-rs2-0(rs1)',NULL,0x4200202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoswap.d','rd-rs2-0(rs1)','rv64',0x0800302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoswap.d.aq','rd-rs2-0(rs1)','rv64',0x0c00302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoswap.d.aqrl','rd-rs2-0(rs1)','rv64',0x0e00302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoswap.d.rl','rd-rs2-0(rs1)','rv64',0x0a00302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoswap.w','rd-rs2-0(rs1)',NULL,0x0800202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoswap.w.aq','rd-rs2-0(rs1)',NULL,0x0c00202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoswap.w.aqrl','rd-rs2-0(rs1)',NULL,0x0e00202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoswap.w.rl','rd-rs2-0(rs1)',NULL,0x0a00202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoxor.d','rd-rs2-0(rs1)','rv64',0x2000302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoxor.d.aq','rd-rs2-0(rs1)','rv64',0x2400302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoxor.d.aqrl','rd-rs2-0(rs1)','rv64',0x2600302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoxor.d.rl','rd-rs2-0(rs1)','rv64',0x2200302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoxor.w','rd-rs2-0(rs1)',NULL,0x2000202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoxor.w.aq','rd-rs2-0(rs1)',NULL,0x2400202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoxor.w.aqrl','rd-rs2-0(rs1)',NULL,0x2600202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('amoxor.w.rl','rd-rs2-0(rs1)',NULL,0x2200202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('and','rd-rs1-rs2',NULL,0x00007033,0xfe00707f,'I');
INSERT INTO instruction VALUES('andi','rd-rs1-i:imm',NULL,0x00007013,0x0000707f,'I');
INSERT INTO instruction VALUES('auipc','rd-u:imm',NULL,0x00000017,0x0000007f,'I');
INSERT INTO instruction VALUES('beq','rs1-rs2-b:imm',NULL,0x00000063,0x0000707f,'I');
INSERT INTO instruction VALUES('bge','rs1-rs2-b:imm',NULL,0x00005063,0x0000707f,'I');
INSERT INTO instruction VALUES('bgeu','rs1-rs2-b:imm',NULL,0x00007063,0x0000707f,'I');
INSERT INTO instruction VALUES('blt','rs1-rs2-b:imm',NULL,0x00004063,0x0000707f,'I');
INSERT INTO instruction VALUES('bltu','rs1-rs2-b:imm',NULL,0x00006063,0x0000707f,'I');
INSERT INTO instruction VALUES('bne','rs1-rs2-b:imm',NULL,0x00001063,0x0000707f,'I');
INSERT INTO instruction VALUES('csrrc','rd-csr-rs1',NULL,0x00003073,0x0000707f,'Zicsr');
INSERT INTO instruction VALUES('csrrci','rd-csr-c:imm',NULL,0x00007073,0x0000707f,'Zicsr');
INSERT INTO instruction VALUES('csrrs','rd-csr-rs1',NULL,0x00002073,0x0000707f,'Zicsr');
INSERT INTO instruction VALUES('csrrsi','rd-csr-c:imm',NULL,0x00006073,0x0000707f,'Zicsr');
INSERT INTO instruction VALUES('csrrw','rd-csr-rs1',NULL,0x00001073,0x0000707f,'Zicsr');
INSERT INTO instruction VALUES('csrrwi','rd-csr-c:imm',NULL,0x00005073,0x0000707f,'Zicsr');
INSERT INTO instruction VALUES('div','rd-rs1-rs2',NULL,0x02004033,0xfe00707f,'M');
INSERT INTO instruction VALUES('divu','rd-rs1-rs2',NULL,0x02005033,0xfe00707f,'M');
INSERT INTO instruction VALUES('divuw','rd-rs1-rs2','rv64',0x0200503b,0xfe00707f,'M');
INSERT INTO instruction VALUES('divw','rd-rs1-rs2','rv64',0x0200403b,0xfe00707f,'M');
INSERT INTO instruction VALUES('ebreak',NULL,NULL,0x00100073,0xffffffff,'I');
INSERT INTO instruction VALUES('ecall',NULL,NULL,0x00000073,0xffffffff,'I');
INSERT INTO instruction VALUES('fence','fencep-fences',NULL,0x0000000f,0x0000707f,'I'); 
INSERT INTO instruction VALUES('fence.i',NULL,NULL,0x0000100f,0xffffffff,'Zifencei');
INSERT INTO instruction VALUES('fence.tso',NULL,NULL,0x8330000f,0xffffffff,'Ztso');


INSERT INTO instruction VALUES('fadd.s','fd-fs1-fs2-rm',NULL,0x00000053,0xfe00007f,'F');
INSERT INTO instruction VALUES('fclass.s','rd-fs1',NULL,0xe0001053,0xfff0707f,'F');
INSERT INTO instruction VALUES('fcvt.l.s','rd-fs1-rm','rv64',0xc0200053,0xfff0007f,'F');
INSERT INTO instruction VALUES('fcvt.lu.s','rd-fs1-rm','rv64',0xc0300053,0xfff0007f,'F');
INSERT INTO instruction VALUES('fcvt.s.l','fd-rs1-rm','rv64',0xd0200053,0xfff0007f,'F');
INSERT INTO instruction VALUES('fcvt.s.lu','rd-fs1-rm','rv64',0xd0300053,0xfff0007f,'F');
INSERT INTO instruction VALUES('fcvt.s.w','fd-rs1-rm',NULL,0xd0000053,0xfff0007f,'F');
INSERT INTO instruction VALUES('fcvt.s.wu','fd-rs1-rm',NULL,0xd0100053,0xfff0007f,'F');
INSERT INTO instruction VALUES('fcvt.w.s','rd-fs1-rm',NULL,0xc0000053,0xfff0007f,'F');
INSERT INTO instruction VALUES('fcvt.wu.s','rd-fs1-rm',NULL,0xc0100053,0xfff0007f,'F');
INSERT INTO instruction VALUES('fdiv.s','fd-fs1-fs2-rm',NULL,0x18000053,0xfe00007f,'F');
INSERT INTO instruction VALUES('fdiv.s','fd-fs1-fs2-rm',NULL,0x18000053,0xfe00007f,'F');
INSERT INTO instruction VALUES('feq.s','rd-fs1-fs2',NULL,0xa0002053,0xfe00707f,'F');
INSERT INTO instruction VALUES('fle.s','rd-fs1-fs2',NULL,0xa0000053,0xfe00707f,'F');
INSERT INTO instruction VALUES('flt.s','rd-fs1-fs2',NULL,0xa0001053,0xfe00707f,'F');
INSERT INTO instruction VALUES('flw','fd-i:imm(rs1)',NULL,0x00002007,0x0000707f,'F');
INSERT INTO instruction VALUES('fmadd.s','fd-fs1-fs2-fs3-rm',NULL,0x00000043,0x0600007f,'F');
INSERT INTO instruction VALUES('fmax.s','fd-fs1-fs2',NULL,0x28001053,0xfe00707f,'F');
INSERT INTO instruction VALUES('fmin.s','fd-fs1-fs2',NULL,0x28000053,0xfe00707f,'F');
INSERT INTO instruction VALUES('fmsub.s','fd-fs1-fs2-fs3-rm',NULL,0x00000047,0x0600007f,'F');
INSERT INTO instruction VALUES('fmul.s','fd-fs1-fs2-rm',NULL,0x10000053,0xfe00007f,'F');
INSERT INTO instruction VALUES('fmv.w.x','rd-fs1',NULL,0xf0000053,0xfff0707f,'F');
INSERT INTO instruction VALUES('fmv.x.w','fd-rs1-rm',NULL,0xe0000053,0xfff0707f,'F');
INSERT INTO instruction VALUES('fnmadd.s','fd-fs1-fs2-fs3-rm',NULL,0x0000004f,0x0600007f,'F');
INSERT INTO instruction VALUES('fnmsub.s','fd-fs1-fs2-fs3-rm',NULL,0x0000004b,0x0600007f,'F');
INSERT INTO instruction VALUES('fsgnj.s','fd-fs1-fs2',NULL,0x20000053,0xfe00707f,'F');
INSERT INTO instruction VALUES('fsgnjn.s','fd-fs1-fs2',NULL,0x20001053,0xfe00707f,'F');
INSERT INTO instruction VALUES('fsgnjx.s','fd-fs1-fs2',NULL,0x20002053,0xfe00707f,'F');     
INSERT INTO instruction VALUES('fsqrt.s','fd-fs1-rm',NULL,0x58000053,0xfff0007f,'F');
INSERT INTO instruction VALUES('fsub.s','fd-fs1-fs2-rm',NULL,0x08000053,0xfe00007f,'F');
INSERT INTO instruction VALUES('fsw','fs2-s:imm(rs1)',NULL,0x00002027,0x0000707f,'F');


INSERT INTO instruction VALUES('fld','fd-i:imm(rs1)',NULL,0x00003007,0x0000707f,'D');
INSERT INTO instruction VALUES('fsd','fs2-s:imm(rs1)',NULL,0x00003027,0x0000707f,'D');
INSERT INTO instruction VALUES('fmadd.d','fd-fs1-fs2-fs3-rm',NULL,0x02000043,0x0600007f,'D');
INSERT INTO instruction VALUES('fmsub.d','fd-fs1-fs2-fs3-rm',NULL,0x02000047,0x0600007f,'D');
INSERT INTO instruction VALUES('fnmsub.d','fd-fs1-fs2-fs3-rm',NULL,0x0200004b,0x0600007f,'D');
INSERT INTO instruction VALUES('fnmadd.d','fd-fs1-fs2-fs3-rm',NULL,0x0200004f,0x0600007f,'D');
INSERT INTO instruction VALUES('fadd.d','fd-fs1-fs2-rm',NULL,0x02000053,0xfe00007f,'D');
INSERT INTO instruction VALUES('fsub.d','fd-fs1-fs2-rm',NULL,0x0a000053,0xfe00007f,'D');
INSERT INTO instruction VALUES('fmul.d','fd-fs1-fs2-rm',NULL,0x12000053,0xfe00007f,'D');
INSERT INTO instruction VALUES('fdiv.d','fd-fs1-fs2-rm',NULL,0x1a000053,0xfe00007f,'D');
INSERT INTO instruction VALUES('fsqrt.d','fd-fs1-rm',NULL,0x5a000053,0xfff0007f,'D');
INSERT INTO instruction VALUES('fsgnj.d','fd-fs1-fs2',NULL,0x22000053,0xfe00707f,'D');
INSERT INTO instruction VALUES('fsgnjn.d','fd-fs1-fs2',NULL,0x22001053,0xfe00707f,'D');
INSERT INTO instruction VALUES('fsgnjx.d','fd-fs1-fs2',NULL,0x22002053,0xfe00707f,'D');  
INSERT INTO instruction VALUES('fmax.d','fd-fs1-fs2',NULL,0x2a001053,0xfe00707f,'D');
INSERT INTO instruction VALUES('fmin.d','fd-fs1-fs2',NULL,0x2a000053,0xfe00707f,'D');
INSERT INTO instruction VALUES('fcvt.s.d','fd-fs1-rm',NULL,0x40100053,0xfff0007f,'D');
INSERT INTO instruction VALUES('fcvt.d.s','fd-fs1-rm',NULL,0x42000053,0xfff0007f,'D');
INSERT INTO instruction VALUES('feq.d','rd-fs1-fs2',NULL,0xa2002053,0xfe00707f,'D');
INSERT INTO instruction VALUES('flt.d','rd-fs1-fs2',NULL,0xa2001053,0xfe00707f,'D');
INSERT INTO instruction VALUES('fle.d','rd-fs1-fs2',NULL,0xa2000053,0xfe00707f,'D');
INSERT INTO instruction VALUES('fclass.s','rd-fs1',NULL,0xe2001053,0xfff0707f,'D');
INSERT INTO instruction VALUES('fcvt.w.d','rd-fs1-rm',NULL,0xc2000053,0xfff0007f,'D');
INSERT INTO instruction VALUES('fcvt.wu.d','rd-fs1-rm',NULL,0xc2100053,0xfff0007f,'D');
INSERT INTO instruction VALUES('fcvt.d.w','fd-rs1-rm',NULL,0xd2000053,0xfff0007f,'D');
INSERT INTO instruction VALUES('fcvt.d.wu','fd-rs1-rm',NULL,0xd2100053,0xfff0007f,'D');

INSERT INTO instruction VALUES('fcvt.l.d','fd-rs1-rm',NULL,0xa2200053,0xfff0007f,'D');
INSERT INTO instruction VALUES('fcvt.lu.d','fd-rs1-rm',NULL,0xa2300053,0xfff0007f,'D');
INSERT INTO instruction VALUES('fmv.x.d','fd-rs1',NULL,0xe2000053,0xfff0707f,'D');
INSERT INTO instruction VALUES('fcvt.d.l','fd-rs1-rm',NULL,0xd2200053,0xfff0007f,'D');
INSERT INTO instruction VALUES('fcvt.d.lu','fd-rs1-rm',NULL,0xd2300053,0xfff0007f,'D');
INSERT INTO instruction VALUES('fmv.d.x','fd-rs1',NULL,0xf2000053,0xfff0707f,'D');


INSERT INTO instruction VALUES('flq','fd-i:imm(rs1)',NULL,0x00004007,0x0000707f,'Q');
INSERT INTO instruction VALUES('fsq','fs2-s:imm(rs1)',NULL,0x00004027,0x0000707f,'Q');
INSERT INTO instruction VALUES('fmadd.q','fd-fs1-fs2-fs3-rm',NULL,0x06000043,0x0600007f,'Q');
INSERT INTO instruction VALUES('fmsub.q','fd-fs1-fs2-fs3-rm',NULL,0x06000047,0x0600007f,'Q');
INSERT INTO instruction VALUES('fnmsub.q','fd-fs1-fs2-fs3-rm',NULL,0x0600004b,0x0600007f,'Q');
INSERT INTO instruction VALUES('fnmadd.q','fd-fs1-fs2-fs3-rm',NULL,0x0600004f,0x0600007f,'Q');
INSERT INTO instruction VALUES('fadd.q','fd-fs1-fs2-rm',NULL,0x03000053,0xfe00007f,'Q');
INSERT INTO instruction VALUES('fsub.q','fd-fs1-fs2-rm',NULL,0x0e000053,0xfe00007f,'Q');
INSERT INTO instruction VALUES('fmul.q','fd-fs1-fs2-rm',NULL,0x16000053,0xfe00007f,'Q');
INSERT INTO instruction VALUES('fdiv.q','fd-fs1-fs2-rm',NULL,0x1e000053,0xfe00007f,'Q');
INSERT INTO instruction VALUES('fsqrt.q','fd-fs1-rm',NULL,0x5e000053,0xfff0007f,'Q');
INSERT INTO instruction VALUES('fsgnj.q','fd-fs1-fs2',NULL,0x26000053,0xfe00707f,'Q');
INSERT INTO instruction VALUES('fsgnjn.q','fd-fs1-fs2',NULL,0x26001053,0xfe00707f,'Q');
INSERT INTO instruction VALUES('fsgnjx.q','fd-fs1-fs2',NULL,0x26002053,0xfe00707f,'Q');  
INSERT INTO instruction VALUES('fmax.q','fd-fs1-fs2',NULL,0x2e001053,0xfe00707f,'Q');
INSERT INTO instruction VALUES('fmin.q','fd-fs1-fs2',NULL,0x2e000053,0xfe00707f,'Q');
INSERT INTO instruction VALUES('fcvt.s.q','fd-fs1-rm',NULL,0x40300053,0xfff0007f,'Q');
INSERT INTO instruction VALUES('fcvt.q.s','fd-fs1-rm',NULL,0x46000053,0xfff0007f,'Q');
INSERT INTO instruction VALUES('fcvt.d.q','fd-fs1-rm',NULL,0x42300053,0xfff0007f,'Q');
INSERT INTO instruction VALUES('fcvt.q.d','fd-fs1-rm',NULL,0x46100053,0xfff0007f,'Q');
INSERT INTO instruction VALUES('feq.q','rd-fs1-fs2',NULL,0xa6002053,0xfe00707f,'Q');
INSERT INTO instruction VALUES('flt.q','rd-fs1-fs2',NULL,0xa6001053,0xfe00707f,'Q');
INSERT INTO instruction VALUES('fle.q','rd-fs1-fs2',NULL,0xa6000053,0xfe00707f,'Q');
INSERT INTO instruction VALUES('fclass.s','rd-fs1',NULL,0xe6001053,0xfff0707f,'Q');
INSERT INTO instruction VALUES('fcvt.w.q','rd-fs1-rm',NULL,0xc6000053,0xfff0007f,'Q');
INSERT INTO instruction VALUES('fcvt.wu.q','rd-fs1-rm',NULL,0xc6100053,0xfff0007f,'Q');
INSERT INTO instruction VALUES('fcvt.q.w','fd-rs1-rm',NULL,0xd6000053,0xfff0007f,'Q');
INSERT INTO instruction VALUES('fcvt.q.wu','fd-rs1-rm',NULL,0xd6100053,0xfff0007f,'Q');

INSERT INTO instruction VALUES('fcvt.l.q','fd-rs1-rm',NULL,0xc6200053,0xfff0007f,'Q');
INSERT INTO instruction VALUES('fcvt.lu.q','fd-rs1-rm',NULL,0xc6300053,0xfff0007f,'Q');
INSERT INTO instruction VALUES('fcvt.q.l','fd-rs1-rm',NULL,0xd6200053,0xfff0007f,'Q');
INSERT INTO instruction VALUES('fcvt.q.lu','fd-rs1-rm',NULL,0xd6300053,0xfff0007f,'Q');




INSERT INTO instruction VALUES('jal','rd-j:imm',NULL,0x0000006f,0x0000007f,'I');
INSERT INTO instruction VALUES('jalr','rd-i:imm(rs1)',NULL,0x00000067,0x0000707f,'I');
INSERT INTO instruction VALUES('lb','rd-i:imm(rs1)',NULL,0x00000003,0x0000707f,'I');
INSERT INTO instruction VALUES('lbu','rd-i:imm(rs1)',NULL,0x00004003,0x0000707f,'I');
INSERT INTO instruction VALUES('ld','rd-i:imm(rs1)','rv64',0x00003003,0x0000707f,'I');
INSERT INTO instruction VALUES('lh','rd-i:imm(rs1)',NULL,0x00001003,0x0000707f,'I');
INSERT INTO instruction VALUES('lhu','rd-i:imm(rs1)',NULL,0x00005003,0x0000707f,'I');
INSERT INTO instruction VALUES('lr.d','rd-0(rs1)','rv64',0x1000302f,0xfff0707f,'A');
INSERT INTO instruction VALUES('lr.d.aq','rd-0(rs1)','rv64',0x1400302f,0xfff0707f,'A');
INSERT INTO instruction VALUES('lr.d.aqrl','rd-0(rs1)','rv64',0x1600302f,0xfff0707f,'A');
INSERT INTO instruction VALUES('lr.d.rl','rd-0(rs1)','rv64',0x1200302f,0xfff0707f,'A');
INSERT INTO instruction VALUES('lr.w','rd-0(rs1)',NULL,0x1000202f,0xfff0707f,'A');
INSERT INTO instruction VALUES('lr.w.aq','rd-0(rs1)',NULL,0x1400202f,0xfff0707f,'A');
INSERT INTO instruction VALUES('lr.w.aqrl','rd-0(rs1)',NULL,0x1600202f,0xfff0707f,'A');
INSERT INTO instruction VALUES('lr.w.rl','rd-0(rs1)',NULL,0x1200202f,0xfff0707f,'A');
INSERT INTO instruction VALUES('lui','rd-u:imm',NULL,0x00000037,0x0000007f,'I');
INSERT INTO instruction VALUES('lw','rd-i:imm(rs1)',NULL,0x00002003,0x0000707f,'I');
INSERT INTO instruction VALUES('lwu','rd-i:imm(rs1)','rv64',0x00006003,0x0000707f,'I');
INSERT INTO instruction VALUES('mul','rd-rs1-rs2',NULL,0x02000033,0xfe00707f,'M');
INSERT INTO instruction VALUES('mulh','rd-rs1-rs2',NULL,0x02001033,0xfe00707f,'M');
INSERT INTO instruction VALUES('mulhsu','rd-rs1-rs2',NULL,0x02002033,0xfe00707f,'M');
INSERT INTO instruction VALUES('mulhu','rd-rs1-rs2',NULL,0x02003033,0xfe00707f,'M');
INSERT INTO instruction VALUES('mulw','rd-rs1-rs2','rv64',0x0200003b,0xfe00707f,'M');
INSERT INTO instruction VALUES('nop',NULL,NULL,0x00000013,0xffffffff,'I');
INSERT INTO instruction VALUES('or','rd-rs1-rs2',NULL,0x00006033,0xfe00707f,'I');
INSERT INTO instruction VALUES('ori','rd-rs1-i:imm',NULL,0x00006013,0x0000707f,'I');
INSERT INTO instruction VALUES('pause',NULL,NULL,0x0100000f,0xffffffff,'Zihintpause');
INSERT INTO instruction VALUES('prefetch.i','p:imm(rs1)',NULL,0x00006013,0x01f07fff,'Zicbop');
INSERT INTO instruction VALUES('prefetch.r','p:imm(rs1)',NULL,0x00106013,0x01f07fff,'Zicbop');
INSERT INTO instruction VALUES('prefetch.w','p:imm(rs1)',NULL,0x00306013,0x01f07fff,'Zicbop');
INSERT INTO instruction VALUES('rem','rd-rs1-rs2',NULL,0x02006033,0xfe00707f,'M');
INSERT INTO instruction VALUES('remu','rd-rs1-rs2',NULL,0x02007033,0xfe00707f,'M');
INSERT INTO instruction VALUES('remuw','rd-rs1-rs2','rv64',0x0200703b,0xfe00707f,'M');
INSERT INTO instruction VALUES('remw','rd-rs1-rs2','rv64',0x0200603b,0xfe00707f,'M');
INSERT INTO instruction VALUES('sb','rs2-s:imm(rs1)',NULL,0x00000023,0x0000707f,'I');
INSERT INTO instruction VALUES('sc.d','rd-rs2-0(rs1)','rv64',0x1800302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('sc.d.aq','rd-rs2-0(rs1)','rv64',0x1c00302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('sc.d.aqrl','rd-rs2-0(rs1)','rv64',0x1e00302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('sc.d.rl','rd-rs2-0(rs1)','rv64',0x1a00302f,0xfe00707f,'A');
INSERT INTO instruction VALUES('sc.w','rd-rs2-0(rs1)',NULL,0x1800202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('sc.w.aq','rd-rs2-0(rs1)',NULL,0x1c00202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('sc.w.aqrl','rd-rs2-0(rs1)',NULL,0x1e00202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('sc.w.rl','rd-rs2-0(rs1)',NULL,0x1a00202f,0xfe00707f,'A');
INSERT INTO instruction VALUES('sd','rs2-s:imm(rs1)','rv64',0x00003023,0x0000707f,'I');
INSERT INTO instruction VALUES('sh','rs2-s:imm(rs1)',NULL,0x00001023,0x0000707f,'I');
INSERT INTO instruction VALUES('sll','rd-rs1-rs2',NULL,0x00001033,0xfe00707f,'I');
INSERT INTO instruction VALUES('slli','rd-rs1-<:imm','rv64',0x00001013,0xfc00707f,'I');
INSERT INTO instruction VALUES('slli','rd-rs1->:imm','rv32',0x00001013,0xfc00707f,'I');
INSERT INTO instruction VALUES('slliw','rd-rs1->:imm','rv64',0x0000101b,0xfe00707f,'I');
INSERT INTO instruction VALUES('sllw','rd-rs1-rs2','rv64',0x0000103b,0xfe00707f,'I');
INSERT INTO instruction VALUES('slt','rd-rs1-rs2',NULL,0x00002033,0xfe00707f,'I');
INSERT INTO instruction VALUES('slti','rd-rs1-i:imm',NULL,0x00002013,0x0000707f,'I');
INSERT INTO instruction VALUES('sltiu','rd-rs1-i:imm',NULL,0x00003013,0x0000707f,'I');
INSERT INTO instruction VALUES('sltu','rd-rs1-rs2',NULL,0x00003033,0xfe00707f,'I');
INSERT INTO instruction VALUES('sra','rd-rs1-rs2',NULL,0x40005033,0xfe00707f,'I');
INSERT INTO instruction VALUES('srai','rd-rs1-<:imm','rv64',0x40005013,0xfc00707f,'I');
INSERT INTO instruction VALUES('srai','rd-rs1->:imm','rv32',0x40005013,0xfc00707f,'I');
INSERT INTO instruction VALUES('sraiw','rd-rs1->:imm','rv64',0x4000501b,0xfe00707f,'I');
INSERT INTO instruction VALUES('sraw','rd-rs1-rs2',NULL,0x4000503b,0xfe00707f,'I');
INSERT INTO instruction VALUES('srl','rd-rs1-rs2',NULL,0x00005033,0xfe00707f,'I');
INSERT INTO instruction VALUES('srli','rd-rs1-<:imm','rv64',0x00005013,0xfc00707f,'I');
INSERT INTO instruction VALUES('srli','rd-rs1->:imm','rv32',0x00005013,0xfc00707f,'I');
INSERT INTO instruction VALUES('srliw','rd-rs1->:imm','rv64',0x0000501b,0xfe00707f,'I');
INSERT INTO instruction VALUES('srlw','rd-rs1-rs2',NULL,0x0000503b,0xfe00707f,'I');
INSERT INTO instruction VALUES('sub','rd-rs1-rs2',NULL,0x40000033,0xfe00707f,'I');
INSERT INTO instruction VALUES('subw','rd-rs1-rs2','rv64',0x4000003b,0xfe00707f,'I');
INSERT INTO instruction VALUES('sw','rs2-s:imm(rs1)',NULL,0x00002023,0x0000707f,'I');
INSERT INTO instruction VALUES('unimp',NULL,NULL,0xc0001073,0xffffffff,'Zicsr');
INSERT INTO instruction VALUES('xor','rd-rs1-rs2',NULL,0x00004033,0xfe00707f,'I');
INSERT INTO instruction VALUES('xori','rd-rs1-i:imm',NULL,0x00004013,0x0000707f,'I');

COMMIT;
