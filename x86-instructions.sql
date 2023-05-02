PRAGMA foreign_keys=ON;
BEGIN TRANSACTION;
CREATE TABLE traits_priority ( -- If different traits may work it's important to pick the less complex one.
    priority INTEGER PRIMARY KEY, -- But if complexity is the same we still need some stability.
    name TEXT NOT NULL UNIQUE
);
INSERT INTO traits_priority VALUES(0,'generic_assembler_operand');
INSERT INTO traits_priority VALUES(1,'assembler_operand_of_8bit_instruction');
INSERT INTO traits_priority VALUES(2,'assembler_operand_separate_accumulator');
INSERT INTO traits_priority VALUES(3,'counter_assembler_operand');
INSERT INTO traits_priority VALUES(4,'source_string_operand');
INSERT INTO traits_priority VALUES(5,'destination_string_operand');
INSERT INTO traits_priority VALUES(6,'io_operand');
INSERT INTO traits_priority VALUES(7,'xlat_operand');

CREATE TABLE operands_priority ( -- If different operand may work it's important to pick the less complex one first.
    priority INTEGER PRIMARY KEY, -- But if complexity is the same we still need some stability.
    name TEXT NOT NULL UNIQUE
);

INSERT INTO operands_priority VALUES(0,'imm8');
INSERT INTO operands_priority VALUES(1,'imm16');
INSERT INTO operands_priority VALUES(2,'imm32');
INSERT INTO operands_priority VALUES(3,'imm64');
INSERT INTO operands_priority VALUES(4,'accumulator_register_8bit');
INSERT INTO operands_priority VALUES(5,'accumulator_register_16bit');
INSERT INTO operands_priority VALUES(6,'accumulator_register_32bit');
INSERT INTO operands_priority VALUES(7,'accumulator_register_64bit');
INSERT INTO operands_priority VALUES(8,'counter_register_8bit');
INSERT INTO operands_priority VALUES(9,'data_register_16bit');
INSERT INTO operands_priority VALUES(10,'gp_register_8bit');
INSERT INTO operands_priority VALUES(11,'low_register_8bit');
INSERT INTO operands_priority VALUES(12,'rex_register_8bit');
INSERT INTO operands_priority VALUES(13,'norex_register_8bit');
INSERT INTO operands_priority VALUES(14,'gp_register_16bit');
INSERT INTO operands_priority VALUES(15,'norex_register_16bit');
INSERT INTO operands_priority VALUES(16,'gp_register_32bit');
INSERT INTO operands_priority VALUES(17,'norex_register_32bit');
INSERT INTO operands_priority VALUES(18,'gp_register_64bit');
INSERT INTO operands_priority VALUES(19,'norex_register_64bit');
INSERT INTO operands_priority VALUES(20,'segment_register');
INSERT INTO operands_priority VALUES(21,'segment_register_no_cs');
INSERT INTO operands_priority VALUES(22,'st_register');
INSERT INTO operands_priority VALUES(23,'x87_register');
INSERT INTO operands_priority VALUES(24,'control_register');
INSERT INTO operands_priority VALUES(25,'debug_register');
INSERT INTO operands_priority VALUES(26,'test_register');
INSERT INTO operands_priority VALUES(27,'mmx_register');
INSERT INTO operands_priority VALUES(28,'nozero_mask_register');
INSERT INTO operands_priority VALUES(29,'mask_register');
INSERT INTO operands_priority VALUES(30,'xmm0_register');
INSERT INTO operands_priority VALUES(31,'xmm_register');
INSERT INTO operands_priority VALUES(32,'ymm_register');
INSERT INTO operands_priority VALUES(33,'zmm_register');
INSERT INTO operands_priority VALUES(34,'absolute_address_16bit_memory_8bit');
INSERT INTO operands_priority VALUES(35,'absolute_address_16bit_memory_16bit');
INSERT INTO operands_priority VALUES(36,'absolute_address_16bit_memory_32bit');
INSERT INTO operands_priority VALUES(37,'absolute_address_32bit_memory_8bit');
INSERT INTO operands_priority VALUES(38,'absolute_address_32bit_memory_16bit');
INSERT INTO operands_priority VALUES(39,'absolute_address_32bit_memory_32bit');
INSERT INTO operands_priority VALUES(40,'absolute_address_32bit_memory_64bit');
INSERT INTO operands_priority VALUES(41,'absolute_address_64bit_memory_8bit');
INSERT INTO operands_priority VALUES(42,'absolute_address_64bit_memory_16bit');
INSERT INTO operands_priority VALUES(43,'absolute_address_64bit_memory_32bit');
INSERT INTO operands_priority VALUES(44,'absolute_address_64bit_memory_64bit');
INSERT INTO operands_priority VALUES(45,'string_instruction_source_address_16bit_memory_8bit');
INSERT INTO operands_priority VALUES(46,'string_instruction_source_address_16bit_memory_16bit');
INSERT INTO operands_priority VALUES(47,'string_instruction_source_address_16bit_memory_32bit');
INSERT INTO operands_priority VALUES(48,'string_instruction_source_address_32bit_memory_8bit');
INSERT INTO operands_priority VALUES(49,'string_instruction_source_address_32bit_memory_16bit');
INSERT INTO operands_priority VALUES(50,'string_instruction_source_address_32bit_memory_32bit');
INSERT INTO operands_priority VALUES(51,'string_instruction_source_address_32bit_memory_64bit');
INSERT INTO operands_priority VALUES(52,'string_instruction_source_address_64bit_memory_8bit');
INSERT INTO operands_priority VALUES(53,'string_instruction_source_address_64bit_memory_16bit');
INSERT INTO operands_priority VALUES(54,'string_instruction_source_address_64bit_memory_32bit');
INSERT INTO operands_priority VALUES(55,'string_instruction_source_address_64bit_memory_64bit');
INSERT INTO operands_priority VALUES(56,'string_instruction_destination_address_16bit_memory_8bit');
INSERT INTO operands_priority VALUES(57,'string_instruction_destination_address_16bit_memory_16bit');
INSERT INTO operands_priority VALUES(58,'string_instruction_destination_address_16bit_memory_32bit');
INSERT INTO operands_priority VALUES(59,'string_instruction_destination_address_32bit_memory_8bit');
INSERT INTO operands_priority VALUES(60,'string_instruction_destination_address_32bit_memory_16bit');
INSERT INTO operands_priority VALUES(61,'string_instruction_destination_address_32bit_memory_32bit');
INSERT INTO operands_priority VALUES(62,'string_instruction_destination_address_32bit_memory_64bit');
INSERT INTO operands_priority VALUES(63,'string_instruction_destination_address_64bit_memory_8bit');
INSERT INTO operands_priority VALUES(64,'string_instruction_destination_address_64bit_memory_16bit');
INSERT INTO operands_priority VALUES(65,'string_instruction_destination_address_64bit_memory_32bit');
INSERT INTO operands_priority VALUES(66,'string_instruction_destination_address_64bit_memory_64bit');
INSERT INTO operands_priority VALUES(67,'address_16bit_memory_0bit');
INSERT INTO operands_priority VALUES(68,'address_16bit_memory_8bit');
INSERT INTO operands_priority VALUES(69,'address_16bit_memory_16bit');
INSERT INTO operands_priority VALUES(70,'address_16bit_memory_32bit');
INSERT INTO operands_priority VALUES(71,'address_16bit_memory_48bit');
INSERT INTO operands_priority VALUES(72,'address_16bit_memory_64bit');
INSERT INTO operands_priority VALUES(73,'address_16bit_memory_80bit');
INSERT INTO operands_priority VALUES(74,'address_16bit_memory_112bit');
INSERT INTO operands_priority VALUES(75,'address_16bit_memory_128bit');
INSERT INTO operands_priority VALUES(76,'address_16bit_memory_224bit');
INSERT INTO operands_priority VALUES(77,'address_16bit_memory_256bit');
INSERT INTO operands_priority VALUES(78,'address_16bit_memory_512bit');
INSERT INTO operands_priority VALUES(79,'address_16bit_memory_752bit');
INSERT INTO operands_priority VALUES(80,'address_16bit_memory_864bit');
INSERT INTO operands_priority VALUES(81,'address_16bit_memory_far_ptr_16bit');
INSERT INTO operands_priority VALUES(82,'address_16bit_memory_far_ptr_32bit');
INSERT INTO operands_priority VALUES(83,'address_32bit_memory_0bit');
INSERT INTO operands_priority VALUES(84,'address_32bit_memory_8bit');
INSERT INTO operands_priority VALUES(85,'address_32bit_memory_16bit');
INSERT INTO operands_priority VALUES(86,'address_32bit_memory_32bit');
INSERT INTO operands_priority VALUES(87,'address_32bit_memory_48bit');
INSERT INTO operands_priority VALUES(88,'address_32bit_memory_64bit');
INSERT INTO operands_priority VALUES(89,'address_32bit_memory_80bit');
INSERT INTO operands_priority VALUES(90,'address_32bit_memory_112bit');
INSERT INTO operands_priority VALUES(91,'address_32bit_memory_128bit');
INSERT INTO operands_priority VALUES(92,'address_32bit_memory_224bit');
INSERT INTO operands_priority VALUES(93,'address_32bit_memory_256bit');
INSERT INTO operands_priority VALUES(94,'address_32bit_memory_512bit');
INSERT INTO operands_priority VALUES(95,'address_32bit_memory_752bit');
INSERT INTO operands_priority VALUES(96,'address_32bit_memory_864bit');
INSERT INTO operands_priority VALUES(97,'address_32bit_memory_far_ptr_16bit');
INSERT INTO operands_priority VALUES(98,'address_32bit_memory_far_ptr_32bit');
INSERT INTO operands_priority VALUES(99,'address_32bit_memory_far_ptr_64bit');
INSERT INTO operands_priority VALUES(100,'norex_address_32bit_memory_0bit');
INSERT INTO operands_priority VALUES(101,'norex_address_32bit_memory_8bit');
INSERT INTO operands_priority VALUES(102,'norex_address_32bit_memory_16bit');
INSERT INTO operands_priority VALUES(103,'norex_address_32bit_memory_32bit');
INSERT INTO operands_priority VALUES(104,'norex_address_32bit_memory_48bit');
INSERT INTO operands_priority VALUES(105,'norex_address_32bit_memory_64bit');
INSERT INTO operands_priority VALUES(106,'norex_address_32bit_memory_80bit');
INSERT INTO operands_priority VALUES(107,'norex_address_32bit_memory_112bit');
INSERT INTO operands_priority VALUES(108,'norex_address_32bit_memory_128bit');
INSERT INTO operands_priority VALUES(109,'norex_address_32bit_memory_224bit');
INSERT INTO operands_priority VALUES(110,'norex_address_32bit_memory_256bit');
INSERT INTO operands_priority VALUES(111,'norex_address_32bit_memory_512bit');
INSERT INTO operands_priority VALUES(112,'norex_address_32bit_memory_752bit');
INSERT INTO operands_priority VALUES(113,'norex_address_32bit_memory_864bit');
INSERT INTO operands_priority VALUES(114,'norex_address_32bit_memory_far_ptr_16bit');
INSERT INTO operands_priority VALUES(115,'norex_address_32bit_memory_far_ptr_32bit');
INSERT INTO operands_priority VALUES(116,'norex_address_32bit_memory_far_ptr_64bit');
INSERT INTO operands_priority VALUES(117,'eip_address_32bit_memory_0bit');
INSERT INTO operands_priority VALUES(118,'eip_address_32bit_memory_8bit');
INSERT INTO operands_priority VALUES(119,'eip_address_32bit_memory_16bit');
INSERT INTO operands_priority VALUES(120,'eip_address_32bit_memory_32bit');
INSERT INTO operands_priority VALUES(121,'eip_address_32bit_memory_48bit');
INSERT INTO operands_priority VALUES(122,'eip_address_32bit_memory_64bit');
INSERT INTO operands_priority VALUES(123,'eip_address_32bit_memory_80bit');
INSERT INTO operands_priority VALUES(124,'eip_address_32bit_memory_112bit');
INSERT INTO operands_priority VALUES(125,'eip_address_32bit_memory_128bit');
INSERT INTO operands_priority VALUES(126,'eip_address_32bit_memory_224bit');
INSERT INTO operands_priority VALUES(127,'eip_address_32bit_memory_256bit');
INSERT INTO operands_priority VALUES(128,'eip_address_32bit_memory_512bit');
INSERT INTO operands_priority VALUES(129,'eip_address_32bit_memory_752bit');
INSERT INTO operands_priority VALUES(130,'eip_address_32bit_memory_864bit');
INSERT INTO operands_priority VALUES(131,'eip_address_32bit_memory_far_ptr_16bit');
INSERT INTO operands_priority VALUES(132,'eip_address_32bit_memory_far_ptr_32bit');
INSERT INTO operands_priority VALUES(133,'eip_address_32bit_memory_far_ptr_64bit');
INSERT INTO operands_priority VALUES(134,'address_64bit_memory_0bit');
INSERT INTO operands_priority VALUES(135,'address_64bit_memory_8bit');
INSERT INTO operands_priority VALUES(136,'address_64bit_memory_16bit');
INSERT INTO operands_priority VALUES(137,'address_64bit_memory_32bit');
INSERT INTO operands_priority VALUES(138,'address_64bit_memory_48bit');
INSERT INTO operands_priority VALUES(139,'address_64bit_memory_64bit');
INSERT INTO operands_priority VALUES(140,'address_64bit_memory_80bit');
INSERT INTO operands_priority VALUES(141,'address_64bit_memory_112bit');
INSERT INTO operands_priority VALUES(142,'address_64bit_memory_128bit');
INSERT INTO operands_priority VALUES(143,'address_64bit_memory_224bit');
INSERT INTO operands_priority VALUES(144,'address_64bit_memory_256bit');
INSERT INTO operands_priority VALUES(145,'address_64bit_memory_512bit');
INSERT INTO operands_priority VALUES(146,'address_64bit_memory_752bit');
INSERT INTO operands_priority VALUES(147,'address_64bit_memory_864bit');
INSERT INTO operands_priority VALUES(148,'address_64bit_memory_far_ptr_16bit');
INSERT INTO operands_priority VALUES(149,'address_64bit_memory_far_ptr_32bit');
INSERT INTO operands_priority VALUES(150,'address_64bit_memory_far_ptr_64bit');
INSERT INTO operands_priority VALUES(151,'norex_address_64bit_memory_0bit');
INSERT INTO operands_priority VALUES(152,'norex_address_64bit_memory_8bit');
INSERT INTO operands_priority VALUES(153,'norex_address_64bit_memory_16bit');
INSERT INTO operands_priority VALUES(154,'norex_address_64bit_memory_32bit');
INSERT INTO operands_priority VALUES(155,'norex_address_64bit_memory_48bit');
INSERT INTO operands_priority VALUES(156,'norex_address_64bit_memory_64bit');
INSERT INTO operands_priority VALUES(157,'norex_address_64bit_memory_80bit');
INSERT INTO operands_priority VALUES(158,'norex_address_64bit_memory_112bit');
INSERT INTO operands_priority VALUES(159,'norex_address_64bit_memory_128bit');
INSERT INTO operands_priority VALUES(160,'norex_address_64bit_memory_224bit');
INSERT INTO operands_priority VALUES(161,'norex_address_64bit_memory_256bit');
INSERT INTO operands_priority VALUES(162,'norex_address_64bit_memory_512bit');
INSERT INTO operands_priority VALUES(163,'norex_address_64bit_memory_752bit');
INSERT INTO operands_priority VALUES(164,'norex_address_64bit_memory_864bit');
INSERT INTO operands_priority VALUES(165,'norex_address_64bit_memory_far_ptr_16bit');
INSERT INTO operands_priority VALUES(166,'norex_address_64bit_memory_far_ptr_32bit');
INSERT INTO operands_priority VALUES(167,'norex_address_64bit_memory_far_ptr_64bit');
INSERT INTO operands_priority VALUES(168,'rip_address_64bit_memory_0bit');
INSERT INTO operands_priority VALUES(169,'rip_address_64bit_memory_8bit');
INSERT INTO operands_priority VALUES(170,'rip_address_64bit_memory_16bit');
INSERT INTO operands_priority VALUES(171,'rip_address_64bit_memory_32bit');
INSERT INTO operands_priority VALUES(172,'rip_address_64bit_memory_48bit');
INSERT INTO operands_priority VALUES(173,'rip_address_64bit_memory_64bit');
INSERT INTO operands_priority VALUES(174,'rip_address_64bit_memory_80bit');
INSERT INTO operands_priority VALUES(175,'rip_address_64bit_memory_112bit');
INSERT INTO operands_priority VALUES(176,'rip_address_64bit_memory_128bit');
INSERT INTO operands_priority VALUES(177,'rip_address_64bit_memory_224bit');
INSERT INTO operands_priority VALUES(178,'rip_address_64bit_memory_256bit');
INSERT INTO operands_priority VALUES(179,'rip_address_64bit_memory_512bit');
INSERT INTO operands_priority VALUES(180,'rip_address_64bit_memory_752bit');
INSERT INTO operands_priority VALUES(181,'rip_address_64bit_memory_864bit');
INSERT INTO operands_priority VALUES(182,'rip_address_64bit_memory_far_ptr_16bit');
INSERT INTO operands_priority VALUES(183,'rip_address_64bit_memory_far_ptr_32bit');
INSERT INTO operands_priority VALUES(184,'rip_address_64bit_memory_far_ptr_64bit');
INSERT INTO operands_priority VALUES(185,'xlat_address_16bit_memory_8bit');
INSERT INTO operands_priority VALUES(186,'xlat_address_32bit_memory_8bit');
INSERT INTO operands_priority VALUES(187,'xlat_address_64bit_memory_8bit');


CREATE TABLE traits_information ( -- Different traits support different assembler operands and we need to pick the right one.
    name TEXT NOT NULL,
    allowed_operand TEXT NOT NULL,
    PRIMARY KEY(allowed_operand, name)
    FOREIGN KEY(name) REFERENCES traits_priority(name)
    FOREIGN KEY(allowed_operand) REFERENCES operands_priority(name)
);
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','accumulator_register_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','accumulator_register_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','accumulator_register_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','accumulator_register_64bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','absolute_address_16bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','absolute_address_16bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','absolute_address_16bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','absolute_address_32bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','absolute_address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','absolute_address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','absolute_address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','absolute_address_64bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','absolute_address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','absolute_address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','absolute_address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_16bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_16bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_16bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_16bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_32bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_32bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_64bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_64bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','control_register');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','debug_register');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','eip_address_32bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','eip_address_32bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','eip_address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','eip_address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','eip_address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','gp_register_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','gp_register_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','gp_register_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','gp_register_64bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','imm8');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','imm16');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','imm32');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','imm64');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','low_register_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_address_32bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_address_32bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_address_64bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_address_64bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_register_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_register_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_register_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','norex_register_64bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','rex_register_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','rip_address_64bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','rip_address_64bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','rip_address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','rip_address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','rip_address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','segment_register');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','st_register');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','test_register');
INSERT INTO traits_information VALUES('assembler_operand_separate_accumulator','x87_register');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_16bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_16bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_16bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_16bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_32bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_32bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_64bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_64bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','eip_address_32bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','eip_address_32bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','eip_address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','eip_address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','eip_address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','gp_register_8bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','gp_register_16bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','gp_register_32bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','gp_register_64bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','imm8');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','imm16');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','imm32');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','imm64');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','low_register_8bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_address_32bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_address_32bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_address_64bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_address_64bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_register_8bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_register_16bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_register_32bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','norex_register_64bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','rex_register_8bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','rip_address_64bit_memory_0bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','rip_address_64bit_memory_8bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','rip_address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','rip_address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('assembler_operand_of_8bit_instruction','rip_address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('counter_assembler_operand','counter_register_8bit');
INSERT INTO traits_information VALUES('counter_assembler_operand','imm8');
INSERT INTO traits_information VALUES('destination_string_operand','string_instruction_destination_address_16bit_memory_8bit');
INSERT INTO traits_information VALUES('destination_string_operand','string_instruction_destination_address_16bit_memory_16bit');
INSERT INTO traits_information VALUES('destination_string_operand','string_instruction_destination_address_16bit_memory_32bit');
INSERT INTO traits_information VALUES('destination_string_operand','string_instruction_destination_address_32bit_memory_8bit');
INSERT INTO traits_information VALUES('destination_string_operand','string_instruction_destination_address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('destination_string_operand','string_instruction_destination_address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('destination_string_operand','string_instruction_destination_address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('destination_string_operand','string_instruction_destination_address_64bit_memory_8bit');
INSERT INTO traits_information VALUES('destination_string_operand','string_instruction_destination_address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('destination_string_operand','string_instruction_destination_address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('destination_string_operand','string_instruction_destination_address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_0bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_48bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_80bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_112bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_128bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_224bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_256bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_512bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_752bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_864bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_far_ptr_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_16bit_memory_far_ptr_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_0bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_48bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_80bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_112bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_128bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_224bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_256bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_512bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_752bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_864bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_far_ptr_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_far_ptr_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_32bit_memory_far_ptr_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_0bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_48bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_80bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_112bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_128bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_224bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_256bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_512bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_752bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_864bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_far_ptr_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_far_ptr_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','address_64bit_memory_far_ptr_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_0bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_48bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_80bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_112bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_128bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_224bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_256bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_512bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_752bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_864bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_far_ptr_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_far_ptr_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','eip_address_32bit_memory_far_ptr_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','gp_register_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','gp_register_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','gp_register_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','imm8');
INSERT INTO traits_information VALUES('generic_assembler_operand','imm16');
INSERT INTO traits_information VALUES('generic_assembler_operand','imm32');
INSERT INTO traits_information VALUES('generic_assembler_operand','imm64');
INSERT INTO traits_information VALUES('generic_assembler_operand','mmx_register');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_0bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_48bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_80bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_112bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_128bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_224bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_256bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_512bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_752bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_864bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_far_ptr_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_far_ptr_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_32bit_memory_far_ptr_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_0bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_48bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_80bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_112bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_128bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_224bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_256bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_512bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_752bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_864bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_far_ptr_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_far_ptr_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','norex_address_64bit_memory_far_ptr_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_0bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_48bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_80bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_112bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_128bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_224bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_256bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_512bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_752bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_864bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_far_ptr_16bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_far_ptr_32bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','rip_address_64bit_memory_far_ptr_64bit');
INSERT INTO traits_information VALUES('generic_assembler_operand','segment_register_no_cs');
INSERT INTO traits_information VALUES('generic_assembler_operand','x87_register');
INSERT INTO traits_information VALUES('io_operand','imm8');
INSERT INTO traits_information VALUES('io_operand','data_register_16bit');
INSERT INTO traits_information VALUES('source_string_operand','string_instruction_source_address_16bit_memory_8bit');
INSERT INTO traits_information VALUES('source_string_operand','string_instruction_source_address_16bit_memory_16bit');
INSERT INTO traits_information VALUES('source_string_operand','string_instruction_source_address_16bit_memory_32bit');
INSERT INTO traits_information VALUES('source_string_operand','string_instruction_source_address_32bit_memory_8bit');
INSERT INTO traits_information VALUES('source_string_operand','string_instruction_source_address_32bit_memory_16bit');
INSERT INTO traits_information VALUES('source_string_operand','string_instruction_source_address_32bit_memory_32bit');
INSERT INTO traits_information VALUES('source_string_operand','string_instruction_source_address_32bit_memory_64bit');
INSERT INTO traits_information VALUES('source_string_operand','string_instruction_source_address_64bit_memory_8bit');
INSERT INTO traits_information VALUES('source_string_operand','string_instruction_source_address_64bit_memory_16bit');
INSERT INTO traits_information VALUES('source_string_operand','string_instruction_source_address_64bit_memory_32bit');
INSERT INTO traits_information VALUES('source_string_operand','string_instruction_source_address_64bit_memory_64bit');
INSERT INTO traits_information VALUES('xlat_operand','xlat_address_16bit_memory_8bit');
INSERT INTO traits_information VALUES('xlat_operand','xlat_address_32bit_memory_8bit');
INSERT INTO traits_information VALUES('xlat_operand','xlat_address_64bit_memory_8bit');

CREATE TABLE operand ( -- Some operand sizes may require 'address_size_prefix' or 'data_size_prefix' depending on x86 CPU mode.
    name TEXT NOT NULL,
    operand_source TEXT CHECK ( -- Operands can be encoded in a many different ways on x86.
        operand_source IN (
            'implicit',  -- al/ax/etc.
            'immediate', -- immediate
            'opcode',    -- opcode (dec/inc/pop/push only)
            'reg',       -- reg field in reg/rm
            'rm'         -- rm field in reg/rm
        )
    ) NOT NULL,
    parameter_type TEXT NOT NULL,
    assembler_kind TEXT CHECK(
        assembler_kind IS NULL OR
        assembler_kind IN (
            'legacy',    -- Legacy include 4 submodes: different defaults, the same set of supported instructions.
            'x86_64'     -- x86_64 only have one mode, but that's the one used most often in today's world.
        )
    ) NULL DEFAULT NULL,
    address_size_prefix TEXT CHECK (
        address_size_prefix IS NULL OR
        address_size_prefix IN (
            'address_size_prefix_16bit', -- Address preffix 16 can only be used in legacy mode
            'address_size_prefix_32bit', -- Address preffix 32 can be in legacy and x86_64 modes
            'address_size_prefix_64bit'  -- Address preffix 64 can only be used in x86_64 mode and means NO prefix there
        )
    ) NULL DEFAULT NULL,
    data_size_prefix TEXT CHECK (
        data_size_prefix IS NULL OR
        data_size_prefix IN (
            'data_size_prefix_16bit', -- Data prefix 16 is supported in all modes
            'data_size_prefix_32bit', -- Data prefix 32 is supported in all modes
            'data_size_prefix_64bit'  -- Data prefix 32 is supported in x86_64 mode and requires REX.W prefix there
        )
    ) NULL DEFAULT NULL,
    PRIMARY KEY(name, assembler_kind, address_size_prefix, data_size_prefix, parameter_type)
);
INSERT INTO operand VALUES('#b','implicit','counter_register_8bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('%b','implicit','xlat_address_16bit_memory_8bit','legacy','address_size_prefix_16bit',NULL);
INSERT INTO operand VALUES('%b','implicit','xlat_address_32bit_memory_8bit',NULL,'address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('%b','implicit','xlat_address_64bit_memory_8bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('&w','implicit','data_register_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES(':7','implicit','st_register',NULL,NULL,NULL);
INSERT INTO operand VALUES(':b','implicit','accumulator_register_8bit',NULL,NULL,NULL);
INSERT INTO operand VALUES(':v','implicit','accumulator_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES(':v','implicit','accumulator_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES(':v','implicit','accumulator_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES(':w','implicit','accumulator_register_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES(':z','implicit','accumulator_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES(':z','implicit','accumulator_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('=S','opcode','segment_register_no_cs',NULL,NULL,NULL);
INSERT INTO operand VALUES('=b','opcode','accumulator_register_8bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('=b','opcode','gp_register_8bit','legacy',NULL,NULL);
INSERT INTO operand VALUES('=b','opcode','low_register_8bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('=b','opcode','norex_register_8bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('=b','opcode','rex_register_8bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('=q','opcode','gp_register_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('=q','opcode','norex_register_16bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('=v','opcode','accumulator_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('=v','opcode','accumulator_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('=v','opcode','accumulator_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('=v','opcode','gp_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('=v','opcode','gp_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('=v','opcode','gp_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('=v','opcode','norex_register_16bit','x86_64',NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('=v','opcode','norex_register_32bit','x86_64',NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('=v','opcode','norex_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('=w','opcode','accumulator_register_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('=w','opcode','gp_register_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('=w','opcode','norex_register_16bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Cd','reg','control_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Cq','reg','control_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Dd','reg','debug_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Dq','reg','debug_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Eb','rm','accumulator_register_8bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Eb','rm','gp_register_8bit','legacy',NULL,NULL);
INSERT INTO operand VALUES('Eb','rm','address_16bit_memory_8bit','legacy','address_size_prefix_16bit',NULL);
INSERT INTO operand VALUES('Eb','rm','address_32bit_memory_8bit',NULL,'address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Eb','rm','address_64bit_memory_8bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Eb','rm','eip_address_32bit_memory_8bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Eb','rm','low_register_8bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Eb','rm','norex_address_32bit_memory_8bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Eb','rm','norex_address_64bit_memory_8bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Eb','rm','norex_register_8bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Eb','rm','rex_register_8bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Eb','rm','rip_address_64bit_memory_8bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Ed','rm','accumulator_register_32bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Ed','rm','gp_register_32bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Ed','rm','address_16bit_memory_32bit','legacy',NULL,NULL);
INSERT INTO operand VALUES('Ed','rm','address_32bit_memory_32bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Ed','rm','address_64bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ed','rm','eip_address_32bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ed','rm','norex_address_32bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ed','rm','norex_address_64bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ed','rm','norex_register_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ed','rm','rip_address_64bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Eq','rm','accumulator_register_64bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Eq','rm','gp_register_64bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Eq','rm','address_16bit_memory_64bit','legacy',NULL,NULL);
INSERT INTO operand VALUES('Eq','rm','address_32bit_memory_64bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Eq','rm','address_64bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Eq','rm','eip_address_32bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Eq','rm','norex_address_32bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Eq','rm','norex_address_64bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Eq','rm','norex_register_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Eq','rm','rip_address_64bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ev','rm','accumulator_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Ev','rm','accumulator_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Ev','rm','accumulator_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('Ev','rm','address_16bit_memory_16bit','legacy','address_size_prefix_16bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Ev','rm','address_16bit_memory_32bit','legacy','address_size_prefix_16bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Ev','rm','address_32bit_memory_16bit',NULL,'address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Ev','rm','address_32bit_memory_32bit',NULL,'address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Ev','rm','address_32bit_memory_64bit','x86_64','address_size_prefix_32bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Ev','rm','address_64bit_memory_16bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Ev','rm','address_64bit_memory_32bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Ev','rm','address_64bit_memory_64bit','x86_64','address_size_prefix_64bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Ev','rm','gp_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Ev','rm','gp_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Ev','rm','gp_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('Ev','rm','eip_address_32bit_memory_16bit','x86_64','address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Ev','rm','eip_address_32bit_memory_32bit','x86_64','address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Ev','rm','eip_address_32bit_memory_64bit','x86_64','address_size_prefix_32bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Ev','rm','norex_address_32bit_memory_16bit','x86_64','address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Ev','rm','norex_address_32bit_memory_32bit','x86_64','address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Ev','rm','norex_address_32bit_memory_64bit','x86_64','address_size_prefix_32bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Ev','rm','norex_address_64bit_memory_16bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Ev','rm','norex_address_64bit_memory_32bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Ev','rm','norex_address_64bit_memory_64bit','x86_64','address_size_prefix_64bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Ev','rm','norex_register_16bit','x86_64',NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Ev','rm','norex_register_32bit','x86_64',NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Ev','rm','norex_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('Ev','rm','rip_address_64bit_memory_16bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Ev','rm','rip_address_64bit_memory_32bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Ev','rm','rip_address_64bit_memory_64bit','x86_64','address_size_prefix_64bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Ew','rm','accumulator_register_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Ew','rm','gp_register_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Ew','rm','address_16bit_memory_16bit','legacy',NULL,NULL);
INSERT INTO operand VALUES('Ew','rm','address_32bit_memory_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Ew','rm','address_64bit_memory_16bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ew','rm','eip_address_32bit_memory_16bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ew','rm','norex_address_32bit_memory_16bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ew','rm','norex_address_64bit_memory_16bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ew','rm','norex_register_16bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ew','rm','rip_address_64bit_memory_16bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Gb','reg','accumulator_register_8bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Gb','reg','gp_register_8bit','legacy',NULL,NULL);
INSERT INTO operand VALUES('Gb','reg','low_register_8bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Gb','reg','norex_register_8bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Gb','reg','rex_register_8bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Gd','reg','accumulator_register_32bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Gd','reg','gp_register_32bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Gd','reg','norex_register_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Gq','reg','accumulator_register_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Gq','reg','gp_register_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Gq','reg','norex_register_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Gv','reg','accumulator_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Gv','reg','accumulator_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Gv','reg','accumulator_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('Gv','reg','gp_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Gv','reg','gp_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Gv','reg','gp_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('Gv','reg','norex_register_16bit','x86_64',NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Gv','reg','norex_register_32bit','x86_64',NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Gv','reg','norex_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('Gw','reg','accumulator_register_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Gw','reg','gp_register_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Gw','reg','norex_register_16bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Gz','reg','accumulator_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Gz','reg','accumulator_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Gz','reg','gp_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Gz','reg','gp_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Gz','reg','norex_register_16bit','x86_64',NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Gz','reg','norex_register_32bit','x86_64',NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Ib','immediate','imm8',NULL,NULL,NULL);
INSERT INTO operand VALUES('Id','immediate','imm32',NULL,NULL,NULL);
INSERT INTO operand VALUES('Iq','immediate','imm64',NULL,NULL,NULL);
INSERT INTO operand VALUES('Iw','immediate','imm16',NULL,NULL,NULL);
INSERT INTO operand VALUES('Iv','immediate','imm16',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Iv','immediate','imm32',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Iv','immediate','imm64','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('Iz','immediate','imm16',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Iz','immediate','imm32',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Iz','immediate','imm32','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('M','rm','address_16bit_memory_0bit','legacy','address_size_prefix_16bit',NULL);
INSERT INTO operand VALUES('M','rm','address_32bit_memory_0bit',NULL,'address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('M','rm','address_64bit_memory_0bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('M','rm','eip_address_32bit_memory_0bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('M','rm','norex_address_32bit_memory_0bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('M','rm','norex_address_64bit_memory_0bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('M','rm','rip_address_64bit_memory_0bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('M7','rm','address_16bit_memory_80bit','legacy','address_size_prefix_16bit',NULL);
INSERT INTO operand VALUES('M7','rm','address_32bit_memory_80bit',NULL,'address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('M7','rm','address_64bit_memory_80bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('M7','rm','eip_address_32bit_memory_80bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('M7','rm','norex_address_32bit_memory_80bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('M7','rm','norex_address_64bit_memory_80bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('M7','rm','rip_address_64bit_memory_80bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('M112/224','rm','address_16bit_memory_112bit','legacy','address_size_prefix_16bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M112/224','rm','address_16bit_memory_224bit','legacy','address_size_prefix_16bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M112/224','rm','address_32bit_memory_112bit',NULL,'address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M112/224','rm','address_32bit_memory_224bit',NULL,'address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M112/224','rm','address_64bit_memory_112bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M112/224','rm','address_64bit_memory_224bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M112/224','rm','eip_address_32bit_memory_112bit','x86_64','address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M112/224','rm','eip_address_32bit_memory_224bit','x86_64','address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M112/224','rm','norex_address_32bit_memory_112bit','x86_64','address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M112/224','rm','norex_address_32bit_memory_224bit','x86_64','address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M112/224','rm','norex_address_64bit_memory_112bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M112/224','rm','norex_address_64bit_memory_224bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M112/224','rm','rip_address_64bit_memory_112bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M112/224','rm','rip_address_64bit_memory_224bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M512','rm','address_16bit_memory_512bit','legacy','address_size_prefix_16bit',NULL);
INSERT INTO operand VALUES('M512','rm','address_32bit_memory_512bit',NULL,'address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('M512','rm','address_64bit_memory_512bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('M512','rm','eip_address_32bit_memory_512bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('M512','rm','norex_address_32bit_memory_512bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('M512','rm','norex_address_64bit_memory_512bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('M512','rm','rip_address_64bit_memory_512bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('M752/864','rm','address_16bit_memory_752bit','legacy','address_size_prefix_16bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M752/864','rm','address_16bit_memory_864bit','legacy','address_size_prefix_16bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M752/864','rm','address_32bit_memory_752bit',NULL,'address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M752/864','rm','address_32bit_memory_864bit',NULL,'address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M752/864','rm','address_64bit_memory_752bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M752/864','rm','address_64bit_memory_864bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M752/864','rm','eip_address_32bit_memory_752bit','x86_64','address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M752/864','rm','eip_address_32bit_memory_864bit','x86_64','address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M752/864','rm','norex_address_32bit_memory_752bit','x86_64','address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M752/864','rm','norex_address_32bit_memory_864bit','x86_64','address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M752/864','rm','norex_address_64bit_memory_752bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M752/864','rm','norex_address_64bit_memory_864bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('M752/864','rm','rip_address_64bit_memory_752bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('M752/864','rm','rip_address_64bit_memory_864bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Md','rm','address_16bit_memory_32bit','legacy',NULL,NULL);
INSERT INTO operand VALUES('Md','rm','address_32bit_memory_32bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Md','rm','address_64bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Md','rm','eip_address_32bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Md','rm','norex_address_32bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Md','rm','norex_address_64bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Md','rm','rip_address_64bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Mo','rm','address_16bit_memory_128bit','legacy',NULL,NULL);
INSERT INTO operand VALUES('Mo','rm','address_32bit_memory_128bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Mo','rm','address_64bit_memory_128bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Mo','rm','eip_address_32bit_memory_128bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Mo','rm','norex_address_32bit_memory_128bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Mo','rm','norex_address_64bit_memory_128bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Mo','rm','rip_address_64bit_memory_128bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Mp','rm','address_16bit_memory_far_ptr_16bit','legacy','address_size_prefix_16bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Mp','rm','address_16bit_memory_far_ptr_32bit','legacy','address_size_prefix_16bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Mp','rm','address_32bit_memory_far_ptr_16bit',NULL,'address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Mp','rm','address_32bit_memory_far_ptr_32bit',NULL,'address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Mp','rm','address_32bit_memory_far_ptr_64bit','x86_64','address_size_prefix_32bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Mp','rm','address_64bit_memory_far_ptr_16bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Mp','rm','address_64bit_memory_far_ptr_32bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Mp','rm','address_64bit_memory_far_ptr_64bit','x86_64','address_size_prefix_64bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Mp','rm','eip_address_32bit_memory_far_ptr_16bit','x86_64','address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Mp','rm','eip_address_32bit_memory_far_ptr_32bit','x86_64','address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Mp','rm','eip_address_32bit_memory_far_ptr_64bit','x86_64','address_size_prefix_32bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Mp','rm','norex_address_32bit_memory_far_ptr_16bit','x86_64','address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Mp','rm','norex_address_32bit_memory_far_ptr_32bit','x86_64','address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Mp','rm','norex_address_32bit_memory_far_ptr_64bit','x86_64','address_size_prefix_32bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Mp','rm','norex_address_64bit_memory_far_ptr_16bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Mp','rm','norex_address_64bit_memory_far_ptr_32bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Mp','rm','norex_address_64bit_memory_far_ptr_64bit','x86_64','address_size_prefix_64bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Mp','rm','norex_register_far_ptr_16bit','x86_64',NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Mp','rm','norex_register_far_ptr_32bit','x86_64',NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Mp','rm','norex_register_far_ptr_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('Mp','rm','rip_address_64bit_memory_far_ptr_16bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Mp','rm','rip_address_64bit_memory_far_ptr_32bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Mp','rm','rip_address_64bit_memory_far_ptr_64bit','x86_64','address_size_prefix_64bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Mq','rm','address_16bit_memory_64bit','legacy',NULL,NULL);
INSERT INTO operand VALUES('Mq','rm','address_32bit_memory_64bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Mq','rm','address_64bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Mq','rm','eip_address_32bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Mq','rm','norex_address_32bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Mq','rm','norex_address_64bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Mq','rm','rip_address_64bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Ms','rm','address_16bit_memory_48bit','legacy','address_size_prefix_16bit',NULL);
INSERT INTO operand VALUES('Ms','rm','address_32bit_memory_48bit','legacy','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Ms','rm','address_32bit_memory_80bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Ms','rm','address_64bit_memory_80bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Ms','rm','eip_address_32bit_memory_80bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Ms','rm','norex_address_32bit_memory_80bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Ms','rm','norex_address_64bit_memory_80bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Ms','rm','rip_address_64bit_memory_80bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Mw','rm','address_16bit_memory_16bit','legacy','address_size_prefix_16bit',NULL);
INSERT INTO operand VALUES('Mw','rm','address_32bit_memory_16bit',NULL,'address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Mw','rm','address_64bit_memory_16bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Mw','rm','eip_address_32bit_memory_16bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Mw','rm','norex_address_32bit_memory_16bit','x86_64','address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Mw','rm','norex_address_64bit_memory_16bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Mw','rm','rip_address_64bit_memory_16bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Nq','rm','mmx_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Ob','immediate','absolute_address_16bit_memory_8bit','legacy','address_size_prefix_16bit',NULL);
INSERT INTO operand VALUES('Ob','immediate','absolute_address_32bit_memory_8bit',NULL,'address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Ob','immediate','absolute_address_64bit_memory_8bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Ov','immediate','absolute_address_16bit_memory_16bit','legacy','address_size_prefix_16bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Ov','immediate','absolute_address_16bit_memory_32bit','legacy','address_size_prefix_16bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Ov','immediate','absolute_address_32bit_memory_16bit',NULL,'address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Ov','immediate','absolute_address_32bit_memory_32bit',NULL,'address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Ov','immediate','absolute_address_32bit_memory_64bit','x86_64','address_size_prefix_32bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Ov','immediate','absolute_address_64bit_memory_16bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Ov','immediate','absolute_address_64bit_memory_32bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Ov','immediate','absolute_address_64bit_memory_64bit','x86_64','address_size_prefix_64bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Pd','reg','mmx_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Pq','reg','mmx_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Qd','rm','mmx_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Qd','rm','address_16bit_memory_32bit','legacy',NULL,NULL);
INSERT INTO operand VALUES('Qd','rm','address_32bit_memory_32bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Qd','rm','address_64bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Qd','rm','eip_address_32bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Qd','rm','norex_address_32bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Qd','rm','norex_address_64bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Qd','rm','rip_address_64bit_memory_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Qq','rm','mmx_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Qq','rm','address_16bit_memory_64bit','legacy',NULL,NULL);
INSERT INTO operand VALUES('Qq','rm','address_32bit_memory_64bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Qq','rm','address_64bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Qq','rm','eip_address_32bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Qq','rm','norex_address_32bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Qq','rm','norex_address_64bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Qq','rm','rip_address_64bit_memory_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('R7','rm','st_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('R7','rm','x87_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Rd','rm','accumulator_register_32bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Rd','rm','gp_register_32bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Rd','rm','norex_register_32bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Rq','rm','accumulator_register_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Rq','rm','gp_register_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Rq','rm','norex_register_64bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Rv','rm','accumulator_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Rv','rm','accumulator_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Rv','rm','accumulator_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('Rv','rm','gp_register_16bit',NULL,NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Rv','rm','gp_register_32bit',NULL,NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Rv','rm','gp_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('Rv','rm','norex_register_16bit','x86_64',NULL,'data_size_prefix_16bit');
INSERT INTO operand VALUES('Rv','rm','norex_register_32bit','x86_64',NULL,'data_size_prefix_32bit');
INSERT INTO operand VALUES('Rv','rm','norex_register_64bit','x86_64',NULL,'data_size_prefix_64bit');
INSERT INTO operand VALUES('Rw','rm','accumulator_register_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Rw','rm','gp_register_16bit',NULL,NULL,NULL);
INSERT INTO operand VALUES('Rw','rm','norex_register_16bit','x86_64',NULL,NULL);
INSERT INTO operand VALUES('Sw','reg','segment_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Td','reg','test_register',NULL,NULL,NULL);
INSERT INTO operand VALUES('Xb','implicit','string_instruction_source_address_16bit_memory_8bit','legacy','address_size_prefix_16bit',NULL);
INSERT INTO operand VALUES('Xb','implicit','string_instruction_source_address_32bit_memory_8bit',NULL,'address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Xb','implicit','string_instruction_source_address_64bit_memory_8bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Xv','implicit','string_instruction_source_address_16bit_memory_16bit','legacy','address_size_prefix_16bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Xv','implicit','string_instruction_source_address_16bit_memory_32bit','legacy','address_size_prefix_16bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Xv','implicit','string_instruction_source_address_32bit_memory_16bit',NULL,'address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Xv','implicit','string_instruction_source_address_32bit_memory_32bit',NULL,'address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Xv','implicit','string_instruction_source_address_32bit_memory_64bit','x86_64','address_size_prefix_32bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Xv','implicit','string_instruction_source_address_64bit_memory_16bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Xv','implicit','string_instruction_source_address_64bit_memory_32bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Xv','implicit','string_instruction_source_address_64bit_memory_64bit','x86_64','address_size_prefix_64bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Xz','implicit','string_instruction_source_address_16bit_memory_16bit','legacy','address_size_prefix_16bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Xz','implicit','string_instruction_source_address_16bit_memory_32bit','legacy','address_size_prefix_16bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Xz','implicit','string_instruction_source_address_32bit_memory_16bit',NULL,'address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Xz','implicit','string_instruction_source_address_32bit_memory_32bit',NULL,'address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Xz','implicit','string_instruction_source_address_64bit_memory_16bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Xz','implicit','string_instruction_source_address_64bit_memory_32bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Yb','implicit','string_instruction_destination_address_16bit_memory_8bit','legacy','address_size_prefix_16bit',NULL);
INSERT INTO operand VALUES('Yb','implicit','string_instruction_destination_address_32bit_memory_8bit',NULL,'address_size_prefix_32bit',NULL);
INSERT INTO operand VALUES('Yb','implicit','string_instruction_destination_address_64bit_memory_8bit','x86_64','address_size_prefix_64bit',NULL);
INSERT INTO operand VALUES('Yv','implicit','string_instruction_destination_address_16bit_memory_16bit','legacy','address_size_prefix_16bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Yv','implicit','string_instruction_destination_address_16bit_memory_32bit','legacy','address_size_prefix_16bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Yv','implicit','string_instruction_destination_address_32bit_memory_16bit',NULL,'address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Yv','implicit','string_instruction_destination_address_32bit_memory_32bit',NULL,'address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Yv','implicit','string_instruction_destination_address_32bit_memory_64bit','x86_64','address_size_prefix_32bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Yv','implicit','string_instruction_destination_address_64bit_memory_16bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Yv','implicit','string_instruction_destination_address_64bit_memory_32bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Yv','implicit','string_instruction_destination_address_64bit_memory_64bit','x86_64','address_size_prefix_64bit','data_size_prefix_64bit');
INSERT INTO operand VALUES('Yz','implicit','string_instruction_destination_address_16bit_memory_16bit','legacy','address_size_prefix_16bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Yz','implicit','string_instruction_destination_address_16bit_memory_32bit','legacy','address_size_prefix_16bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Yz','implicit','string_instruction_destination_address_32bit_memory_16bit',NULL,'address_size_prefix_32bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Yz','implicit','string_instruction_destination_address_32bit_memory_32bit',NULL,'address_size_prefix_32bit','data_size_prefix_32bit');
INSERT INTO operand VALUES('Yz','implicit','string_instruction_destination_address_64bit_memory_16bit','x86_64','address_size_prefix_64bit','data_size_prefix_16bit');
INSERT INTO operand VALUES('Yz','implicit','string_instruction_destination_address_64bit_memory_32bit','x86_64','address_size_prefix_64bit','data_size_prefix_32bit');

CREATE TABLE operands ( -- Separated from instructions table mostly to catch mistakes.
                        -- Many combinations are forbidden and it's easier to whitelist them than write aleborate CHECKs.
    short_name TEXT PRIMARY KEY,
    operand0 TEXT CHECK (operand0 IS NULL OR operand0 IN ('%b', '&w', ':b', ':7', ':v', ':w', ':z', '=S', '=b', '=q', '=v', '=w', 'Cd', 'Cq', 'Dd', 'Dq', 'Eb', 'Ed', 'Mp', 'Eq', 'Ev', 'Ew', 'Gb', 'Gd', 'Gq', 'Gv', 'Gw', 'Gz', 'Ib', 'Id', 'Iq', 'Iw', 'Iz', 'M7', 'M112/224', 'M512', 'M752/864', 'M', 'Md', 'Mo', 'Mq', 'Ms', 'Mw', 'Nq', 'Ob', 'Ov', 'Pd', 'Pq', 'Qq', 'R7', 'Rd', 'Rw', 'Rq', 'Sw', 'Td', 'Xb', 'Xv', 'Xz', 'Yb', 'Yv', 'Yz')) NULL DEFAULT NULL,
    operand1 TEXT CHECK (operand1 IS NULL OR operand1 IN ('#b', '%b', '&w', ':7', ':b', ':v', ':z', '=v', 'Cd', 'Cq', 'Dd', 'Dq', 'Eb', 'Ed', 'Ev', 'Ew', 'Eq', 'Gb', 'Gv', 'Gw', 'Ib', 'Iz', 'Iv', 'M', 'M112/224', 'Md', 'Mp', 'Mq', 'Mw', 'Ob', 'Ov', 'Pd', 'Pq', 'Qd', 'Qq', 'R7', 'Rd', 'Rq', 'Rv', 'Rw', 'Sw', 'Td', 'Xb', 'Xv', 'Xz', 'Yb', 'Yz', 'Yv')) NULL DEFAULT NULL,
    operand2 TEXT CHECK (operand2 IS NULL OR operand2 IN ('#b', ':v', 'Ib', 'Iz')) NULL DEFAULT NULL,
    operand3 TEXT CHECK (operand3 IS NULL OR operand3 IN ('#b', 'Gv')) NULL DEFAULT NULL,
    operand4 TEXT CHECK (operand4 IS NULL) NULL DEFAULT NULL
);
INSERT INTO operands VALUES(NULL,NULL,NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('%b','%b',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('&w-:b','&w',':b',NULL,NULL,NULL);
INSERT INTO operands VALUES('&w-:z','&w',':z',NULL,NULL,NULL);
INSERT INTO operands VALUES('&w-Xb','&w','Xb',NULL,NULL,NULL);
INSERT INTO operands VALUES('&w-Xz','&w','Xz',NULL,NULL,NULL);
INSERT INTO operands VALUES(':7-R7',':7','R7',NULL,NULL,NULL);
INSERT INTO operands VALUES(':b-%b',':b','%b',NULL,NULL,NULL);
INSERT INTO operands VALUES(':b-&w',':b','&w',NULL,NULL,NULL);
INSERT INTO operands VALUES(':b-Ib',':b','Ib',NULL,NULL,NULL);
INSERT INTO operands VALUES(':b-Ob',':b','Ob',NULL,NULL,NULL);
INSERT INTO operands VALUES(':b-Xb',':b','Xb',NULL,NULL,NULL);
INSERT INTO operands VALUES(':b-Yb',':b','Yb',NULL,NULL,NULL);
INSERT INTO operands VALUES(':v-Iz',':v','Iz',NULL,NULL,NULL);
INSERT INTO operands VALUES(':v-=v',':v','=v',NULL,NULL,NULL);
INSERT INTO operands VALUES(':v-Ov',':v','Ov',NULL,NULL,NULL);
INSERT INTO operands VALUES(':v-Xv',':v','Xv',NULL,NULL,NULL);
INSERT INTO operands VALUES(':v-Yv',':v','Yv',NULL,NULL,NULL);
INSERT INTO operands VALUES(':w',':w',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES(':z-&w',':z','&w',NULL,NULL,NULL);
INSERT INTO operands VALUES(':z-Ib',':z','Ib',NULL,NULL,NULL);
INSERT INTO operands VALUES('=S','=S',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('=b-Ib','=b','Ib',NULL,NULL,NULL);
INSERT INTO operands VALUES('=q','=q',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('=v','=v',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('=v-:v','=v',':v',NULL,NULL,NULL);
INSERT INTO operands VALUES('=v-Iv','=v','Iv',NULL,NULL,NULL);
INSERT INTO operands VALUES('=w','=w',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Ap','Iw','Iz',NULL,NULL,NULL);
INSERT INTO operands VALUES('Cd-Rd','Cd','Rd',NULL,NULL,NULL);
INSERT INTO operands VALUES('Cq-Rq','Cq','Rq',NULL,NULL,NULL);
INSERT INTO operands VALUES('Dd-Rd','Dd','Rd',NULL,NULL,NULL);
INSERT INTO operands VALUES('Dq-Rq','Dq','Rq',NULL,NULL,NULL);
INSERT INTO operands VALUES('Eb','Eb',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Eb-#b','Eb','#b',NULL,NULL,NULL);
INSERT INTO operands VALUES('Eb-Gb','Eb','Gb',NULL,NULL,NULL);
INSERT INTO operands VALUES('Eb-Ib','Eb','Ib',NULL,NULL,NULL);
INSERT INTO operands VALUES('Eq','Eq',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Ev','Ev',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Ev-#b','Ev','#b',NULL,NULL,NULL);
INSERT INTO operands VALUES('Ev-:v-#b-Gv','Ev',':v','#b','Gv',NULL);
INSERT INTO operands VALUES('Ev-Gv','Ev','Gv',NULL,NULL,NULL);
INSERT INTO operands VALUES('Ev-Gv-Ib','Ev','Gv','Ib',NULL,NULL);
INSERT INTO operands VALUES('Ev-Gv-#b','Ev','Gv','#b',NULL,NULL);
INSERT INTO operands VALUES('Ev-Ib','Ev','Ib',NULL,NULL,NULL);
INSERT INTO operands VALUES('Ev-Iz','Ev','Iz',NULL,NULL,NULL);
INSERT INTO operands VALUES('Ev-Sw','Ev','Sw',NULL,NULL,NULL);
INSERT INTO operands VALUES('Ew','Ew',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Ew-Gw','Ew','Gw',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gb-Eb','Gb','Eb',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gd-Ed','Gd','Ed',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gd-Mq','Gd','Mq',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gd-Rd','Gd','Rd',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gq-Ed','Gq','Ed',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gq-Rd','Gq','Rd',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gv-Eb','Gv','Eb',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gv-Ev','Gv','Ev',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gv-Ev-:v-#b','Gv','Ev',':v','#b',NULL);
INSERT INTO operands VALUES('Gv-Ev-Ib','Gv','Ev','Ib',NULL,NULL);
INSERT INTO operands VALUES('Gv-Ev-Iz','Gv','Ev','Iz',NULL,NULL);
INSERT INTO operands VALUES('Gv-Ew','Gv','Ew',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gv-M','Gv','M',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gv-Mp','Gv','Mp',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gv-Mw','Gv','Mw',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gv-Rv','Gv','Rv',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gw-Ew','Gw','Ew',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gw-Md','Gw','Md',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gw-Rw','Gw','Rw',NULL,NULL,NULL);
INSERT INTO operands VALUES('Gz-Ew','Gz','Ew',NULL,NULL,NULL);
INSERT INTO operands VALUES('Ib','Ib',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Ib-:b','Ib',':b',NULL,NULL,NULL);
INSERT INTO operands VALUES('Ib-:z','Ib',':z',NULL,NULL,NULL);
INSERT INTO operands VALUES('Id','Id',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Iq','Iq',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Iw','Iw',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Iw-Ib','Iw','Ib',NULL,NULL,NULL);
INSERT INTO operands VALUES('Iz','Iz',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('M7','M7',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('M112/224','M112/224',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('M512','M512',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('M752/864','M752/864',NULL,NULL,NULL,NULL); 
INSERT INTO operands VALUES('M','M',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Md','Md',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Mo','Mo',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Mp','Mp',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Mq','Mq',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Ms','Ms',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Mw','Mw',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Ob-:b','Ob',':b',NULL,NULL,NULL);
INSERT INTO operands VALUES('Ov-:v','Ov',':v',NULL,NULL,NULL);
INSERT INTO operands VALUES('R7','R7',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('R7-:7','R7',':7',NULL,NULL,NULL);
INSERT INTO operands VALUES('Rw','Rw',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Rd','Rd',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Rd-Cd','Rd','Cd',NULL,NULL,NULL);
INSERT INTO operands VALUES('Rd-Dd','Rd','Dd',NULL,NULL,NULL);
INSERT INTO operands VALUES('Rd-Td','Rd','Td',NULL,NULL,NULL);
INSERT INTO operands VALUES('Rq','Rq',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Rq-Cq','Rq','Cq',NULL,NULL,NULL);
INSERT INTO operands VALUES('Rq-Dq','Rq','Dq',NULL,NULL,NULL);
INSERT INTO operands VALUES('Sw-Ev','Sw','Ev',NULL,NULL,NULL);
INSERT INTO operands VALUES('Td-Rd','Td','Rd',NULL,NULL,NULL);
INSERT INTO operands VALUES('Xb','Xb',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Xb-Yb','Xb','Yb',NULL,NULL,NULL);
INSERT INTO operands VALUES('Xv','Xv',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Xv-Yv','Xv','Yv',NULL,NULL,NULL);
INSERT INTO operands VALUES('Yb','Yb',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Yb-&w','Yb','&w',NULL,NULL,NULL);
INSERT INTO operands VALUES('Yb-:b','Yb',':b',NULL,NULL,NULL);
INSERT INTO operands VALUES('Yb-Xb','Yb','Xb',NULL,NULL,NULL);
INSERT INTO operands VALUES('Yv','Yv',NULL,NULL,NULL,NULL);
INSERT INTO operands VALUES('Yv-:v','Yv',':v',NULL,NULL,NULL);
INSERT INTO operands VALUES('Yv-Xv','Yv','Xv',NULL,NULL,NULL);
INSERT INTO operands VALUES('Yv-&w','Yv','&w',NULL,NULL,NULL);
INSERT INTO operands VALUES('Yz-&w','Yz','&w',NULL,NULL,NULL);
INSERT INTO operands VALUES('Ed-Pd','Ed','Pd',NULL,NULL,NULL);
INSERT INTO operands VALUES('Eq-Pq','Eq','Pq',NULL,NULL,NULL);
INSERT INTO operands VALUES('Pd-Ed','Pd','Ed',NULL,NULL,NULL);
INSERT INTO operands VALUES('Pq-Eq','Pq','Eq',NULL,NULL,NULL);
INSERT INTO operands VALUES('Nq-Ib','Nq','Ib',NULL,NULL,NULL);
INSERT INTO operands VALUES('Pq-Qd','Pq','Qd',NULL,NULL,NULL);
INSERT INTO operands VALUES('Pq-Qq','Pq','Qq',NULL,NULL,NULL);
INSERT INTO operands VALUES('Qq-Pq','Qq','Pq',NULL,NULL,NULL);

CREATE TABLE instruction (
    name TEXT NOT NULL, -- Instructrion name.
    operands TEXT CHECK ( -- operands. See “AMD64 ArchitectureProgrammer’s Manual. Volume 3.”
        operands IS NULL OR --             “Appendix A   Opcode and Operand Encodings.”
        operands IN (
            '%b',          -- e.e. xlat BYTE PTR ds:[bx]
            '&w-:b',       -- e.g. out dx, al
            '&w-:z',       -- e.g. out dx, ax
            '&w-Xb',       -- e.g. outs dx, BYTE PTR ds:[si]
            '&w-Xz',       -- e.g. outs dx, WORD PTR ds:[si]
            ':7-R7',       -- e.g. fadd st,st(1)
            ':b-%b',       -- e.e. xlat al, BYTE PTR ds:[bx]
            ':b-&w',       -- e.g. in al, dx
            ':z-&w',       -- e.g. in ax, dx
            ':b-Ib',       -- e.g. add al, 1                 Note: :b-Ib comes before Eb-Ib
            ':b-Ob',       -- e.g. mov al, BYTE PTR [1234]         this is important for this case:
            ':b-Xb',       -- e.g. lods al, BYTE PTR ds:[si]       accumulator_register_8bit, imm8
            ':b-Yb',       -- e.g. scas al, BYTE PTR es:[di]
            ':v-=v',       -- e.g. xchg ax, bx
            ':v-Iz',       -- e.g. add ax, 1234
            ':v-Ov',       -- e.g. mov ax, WORD PTR [1234]
            ':v-Xv',       -- e.g. lods ax, WORD PTR ds:[si]
            ':v-Yv',       -- e.g. scas ax, WORD PTR es:[di]
            ':w',          -- e.g. fstsw ax
            ':z-Ib',       -- e.g. in ax, 123
            '=S',          -- e.g. pop fs
            '=b-Ib',       -- e.g. mov al, 1                 Note: only mov
            '=q',          -- e.g. push rax                  Note: only push/pop in x86-64 mode
            '=v',          -- e.g. dec ax                    Note: only dec/inc/push/pop in legacy mode
            '=v-:v',       -- e.g. xchg bx,ax
            '=v-Iv',       -- e.g. mov rax, 12345678         Note: only mov, but supports imm64
            '=w',          -- e.g. pop ax                    Note: even in x86_64 mode
            'Ap',          -- e.g. call imm8:imm16/imm32 
            'Cd-Rd',       -- e.g. mov cr0, eax
            'Cq-Rq',       -- e.g. mov cr0, rax
            'Dd-Rd',       -- e.g. mov dr0, eax
            'Dq-Rq',       -- e.g. mov dr0, rax
            'Eb',          -- e.g. mul al
            'Eb-#b',       -- e.g. shl al, cl
            'Eb-Gb',       -- e.g. add BYTE PTR [bx], al     Note: Eb-Gb comes before Gb-Eb
            'Eb-Ib',       -- e.g. add BYTE PTR [bx], 1            this is important for this case:
            'Ed-Pd',       -- e.g. movd DWORD PTR [bx], mm0 
            'Eq-Pq',       -- e.g. movq QWORD PTR [bx], mm0
            'Pd-Ed',       -- e.g. movd mm0, DWORD PTR [bx]
            'Pq-Eq',       -- e.g. movq mm0, QWORD PTR [bx]
            'Eq',          -- e.g. pop rax                         gp_register_8bit, gp_register_8bit
            'Ev',          -- e.g. mul ax
            'Ev-#b',       -- e.g. shl ax, cl
            'Ev-:v-#b-Gv', -- e.g. ibts WORD PTR [bx], ax, cl, dx
            'Ev-Ib',       -- e.g. add BYTE PTR [bx], 1
            'Ev-Iz',       -- e.g. add WQRD PTR [bx], 1234
            'Ev-Gv',       -- e.g. add WORD PTR [bx], ax
            'Ev-Gv-Ib',    -- e.g. shld WORD PTR [bx], ax, #1
            'Ev-Gv-#b',    -- e.g. shld WORD PTR [bx], ax, cl
            'Ew',          -- e.g. pop WORD PTR [bx]
            'Ew-Gw',       -- e.g. arpl WORD PTR [bx],ax
            'Ev-Sw',       -- e.g. mov ax, fs
            'Gb-Eb',       -- e.g. add al, BYTE PTR [bx]
            'Gd-Ed',       -- e.g. movsxd eax, DWORD PTR [ebx] (not recommended, but works)
            'Gd-Mq',       -- e.g. bound eax, QWORD PTR [bx]
            'Gd-Rd',       -- e.g. lsl eax, ebx
            'Gq-Ed',       -- e.g. movsxd rax, DWORD PTR [ebx]
            'Gq-Rd',       -- e.g. lsl rax, ebx
            'Gv-Eb',       -- e.g. movsx ax, BYTE PTR [bx]
            'Gv-Ev',       -- e.g. add ax, WORD PTR [bx]
            'Gv-Ev-:v-#b', -- e.g. xbts dx, WORD PTR [bx], ax, cl
            'Gv-Ev-Ib',    -- e.g. imul ax, WORD PTR [bx], 123
            'Gv-Ev-Iz',    -- e.g. imul ax, WORD PTR [bx], 1234
            'Gv-Ew',       -- e.g. movsx eax, WORD PTR [bx]
            'Gv-M',        -- e.g. lea ax, [bx+di+1234]
            'Gv-Mp',       -- e.g. lfs ax, DWORD PTR [bx]
            'Gv-Mw',       -- e.e. lsl ax, WORD PTR [bx]
            'Gv-Rv',       -- e.e. ud1 ax, bx
            'Gw-Ew',       -- e.g. movsxd ax, WORD PTR [ebx] (not recommended, but works)
            'Gw-Md',       -- e.g. bound ax, DWORD PTR [bx]
            'Gw-Rw',       -- e.g. lsl ax, bx
            'Gz-Ew',       -- e.g. lar ax, WORD PTR [bx]
            'Ib',          -- e.g. aad 10
            'Ib-:b',       -- e.g. out 123, al
            'Ib-:z',       -- e.g. out 123, ax
            'Iw',          -- e.g. ret 1234
            'Iw-Ib',       -- e.g. enter 1, 2
            'Iz',          -- e.g. push 1234
            'M7',          -- e.g. fld TBYTE PTR [bx]
            'M112/224',    -- e.g. fldenv [bx]
            'M512',        -- e.g. fxsave [bx]
            'M752/864',    -- e.g. frstor [bx]
            'M',           -- e.g. invlpg [bx]
            'Md',          -- e.g. fld DWORD PTR [bx]
            'Mo',          -- e.g. cmpxchg16b OWORD PTR [ebx]
            'Mq',          -- e.g. fld QWORD PTR [bx]
            'Mp',          -- e.g. call FAR PTR [bx]
            'Ms',          -- e.g. lgdt SBYTE PTR [bx]
            'Mw',          -- e.g. smsw WORD PTR [bx]
            'Nq-Ib',       -- e.g. psllw mm0, 1
            'Ob-:b',       -- e.g. mov BYTE PTR [1234], al
            'Ov-:v',       -- e.g. mov WORD PTR [1234], ax
            'Pq-Eq',       -- e.g. movq mm0, QWORD PTR [bx]
            'Pq-Qd',       -- e.g. movq mm0, mm1
            'Pq-Qq',       -- e.g. movq mm0, mm1 
            'Qq-Pq',       -- e.g. movq mm0, mm1
            'R7',          -- e.g. fld st(1)
            'R7-:7',       -- e.g. fadd st(1),st
            'Rd',          -- e.g. smsw eax
            'Rd-Cd',       -- e.g. mov eax, dr0
            'Rd-Dd',       -- e.g. mov eax, dr0
            'Rd-Td',       -- e.g. mov eax, tr7
            'Rq-Cq',       -- e.g. mov rax, dr0
            'Rq-Dq',       -- e.g. mov rax, dr0
            'Rq',          -- e.g. smsw rax
            'Rw',          -- e.g. smsw ax
            'Sw-Ev',       -- e.g. mov fs, ax
            'Td-Rd',       -- e.g. mov tr7, eax
            'Xb',          -- e.g. lods BYTE PTR ds:[si]
            'Xb-Yb',       -- e.g. cmps BYTE PTR ds:[si], BYTE PTR es:[di]
            'Xv',          -- e.g. lods WORD PTR ds:[si]
            'Xv-Yv',       -- e.g. cmps WORD PTR ds:[si], WORD PTR es:[di]
            'Yb',          -- e.g. scas es:[di]
            'Yb-:b',       -- e.g. stos BYTE PTR es:[di], al
            'Yb-&w',       -- e.g. ins BYTE PTR es:[di], dx
            'Yb-Xb',       -- e.g. movs BYTE PTR es:[di], BYTE PTR ds:[si]
            'Yv',          -- e.g. scas WORD PTR es:[di]
            'Yz-&w',       -- e.g. ins WORD PTR es:[di], dx
            'Yv-:v',       -- e.g. stos WORD PTR es:[di], ax
            'Yv-Xv'        -- e.g. movs WORD PTR es:[di], WORD PTR ds:[si]
        )
    ),
    assembler_kind TEXT CHECK( -- Certain instructions can only be used in a specific assembler mode.
        assembler_kind IS NULL OR
        assembler_kind IN (
            'legacy',
            'x86_64'
        )
    ) NULL DEFAULT NULL,
    address_size_prefix TEXT CHECK(     -- Used for specifying data address on 80386.
        address_size_prefix IS NULL OR  -- Used to hadle weird case of jcxz/jecxz/jrcxz instruction.
        address_size_prefix IN (
            'address_size_prefix_16bit',
            'address_size_prefix_32bit',
            'address_size_prefix_64bit'
        )
    ) NULL,
    data_size_prefix TEXT CHECK(      -- Used for specifying data size on 80386.
        data_size_prefix IS NULL OR   -- SIMD extensios reused it as opcode extension.
        data_size_prefix IN (         -- cand be limited by operand: e.g. 16ᵇⁱᵗ/32ᵇⁱᵗ/64ᵇⁱᵗ instructions.
            'data_size_prefix_16bit', -- e.g. cmpsw
            'data_size_prefix_32bit', -- e.g. cmpsd
            'data_size_prefix_64bit', -- e.g. cmpsq
            'data_size_prefix',       -- e.g. SSE instructions.
            'no_data_size_prefix'     -- e.g. MMX instructions.
        )
    ) NULL,
    repx_prefix TEXT CHECK (   -- Used with string instructions.
        repx_prefix IS NULL OR -- SIMD extensios reused it as opcode extension.
        repx_prefix IN (       -- e.g. mov al, $0
            '0xf2',            -- e.g. cvtsi2sd
            '0xf3',            -- e.g. cvtsi2ss
            'not_allowed'      -- e.g. cvtpi2ps
        )
    ) NULL,
    opcode_map TEXT CHECK ( -- x86 uses four opcode maps:
        opcode_map IN (
            'primary',   -- 8086/80186 instructions.
            'secondary', -- 80286+ instructions.
            '0x0f_0x38', -- Legacy SIMD instructions with immediates.
            '0x0f_0x3a'  -- Legacy SIMD instructions with immediates.
        )
    ) NOT NULL,
    opcode INTEGER NOT NULL,
    opcode_extension INTEGER NULL DEFAULT NULL, -- Can be extension in ModR/M or immediate, never both.
    cpuid TEXT CHECK (  -- CPUID information (not used by assembler directly).
        cpuid IN (      --     Note: not all instructions sets can be distinguished by CPUID since it's 486+ instruction.
            '8086',       -- 8086/80186 instructions.
            '8087',       -- 8087 instructions.
            '80186',      -- 80186/80188 instructions.
            '80286',      -- 80287 instructions.
            '80287',      -- 80287 instructions.
            '80386',      -- 80386 instructions. rsm was introduced in 386SL thus it's listed here, too.
            '80387',      -- 80387 instructions.
            '80486',      -- 80486 instructions (CPUID is Pentium, but backported to 486DX100).
            'bmi1',       -- bmi1 turns bsf with rep prefix into tzcnt (and adds some other instruction).
            'bmi2',
            'cmov',       -- cmovcc, (Pentium Pro).
            'cmov&x87',   -- fcmovcc, (Pentium Pro and x87 must be present, too).
            'cmpxchg8b',  -- cmpxchg8b (Pentium+).
            'cmpxchg16b', -- cmpxchg16b (x86_64 except early CPUs).
            'fxsr',       -- fxsave/fxrstor (Pentium II Deschutes).
            'lzcnt',      -- lzcnt turns bsr with rep prefix into lzcnt.
            'lahf',       -- only 64 bit version, 32bit version always available!
            'mmx',        -- MMX (Pentium MMX).
            'msr',        -- rdmsr/wrmsr (Pentium+).
            'nop',        -- long nop (Pentium Pro): CPUID.01H.EAX[Bits 11:8] = 0110B or 1111B.
            'pmc',        -- performance monitoring CPUs (Pentium Pro): CPUID.0AH:EAX[7:0] ≠ 0.
            'rdtscp',     -- rdtscp (Opteron, first implementation of x86_64 architecture).
            'sep',        -- sep and extra checks (Pentium II): see http://www.os2museum.com/wp/sysenter-where-are-you/.
            'sse3&x87',   -- SSE3 & x87 (Prescott/Opteron).
            'syscall',    -- K6-2 AMD (ia32 and x86_64 modes) or x86_64 Intel.
            'tsc',        -- rdtsc (Pentium+).
            'x86_64'      -- legacy 8086/80386 instructions extended with a new name in x86_64 mode.
        )
    ) NOT NULL,
    PRIMARY KEY(name, operands, assembler_kind, address_size_prefix, data_size_prefix, repx_prefix, opcode_map, opcode, opcode_extension),
    FOREIGN KEY(operands) REFERENCES operands(short_name)
);
INSERT INTO instruction VALUES('aaa',NULL,'legacy',NULL,NULL,NULL,'primary',0x37,NULL,'8086');

INSERT INTO instruction VALUES('aad',NULL,'legacy',NULL,NULL,NULL,'primary',0xd5,0x0a,'8086');
INSERT INTO instruction VALUES('aad','Ib','legacy',NULL,NULL,NULL,'primary',0xd5,NULL,'8086');

INSERT INTO instruction VALUES('aam',NULL,'legacy',NULL,NULL,NULL,'primary',0xd4,0x0a,'8086');
INSERT INTO instruction VALUES('aam','Ib','legacy',NULL,NULL,NULL,'primary',0xd4,NULL,'8086');

INSERT INTO instruction VALUES('aas',NULL,'legacy',NULL,NULL,NULL,'primary',0x3f,NULL,'8086');

INSERT INTO instruction VALUES('adc',':b-Ib',NULL,NULL,NULL,NULL,'primary',0x14,NULL,'8086');
INSERT INTO instruction VALUES('adc',':v-Iz',NULL,NULL,NULL,NULL,'primary',0x15,NULL,'8086');
INSERT INTO instruction VALUES('adc','Eb-Gb',NULL,NULL,NULL,NULL,'primary',0x10,NULL,'8086');
INSERT INTO instruction VALUES('adc','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0x80,2,'8086');
INSERT INTO instruction VALUES('adc','Ev-Gv',NULL,NULL,NULL,NULL,'primary',0x11,NULL,'8086');
INSERT INTO instruction VALUES('adc','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0x83,2,'8086');
INSERT INTO instruction VALUES('adc','Ev-Iz',NULL,NULL,NULL,NULL,'primary',0x81,2,'8086');
INSERT INTO instruction VALUES('adc','Gb-Eb',NULL,NULL,NULL,NULL,'primary',0x12,NULL,'8086');
INSERT INTO instruction VALUES('adc','Gv-Ev',NULL,NULL,NULL,NULL,'primary',0x13,NULL,'8086');

INSERT INTO instruction VALUES('add',':b-Ib',NULL,NULL,NULL,NULL,'primary',0x04,NULL,'8086');
INSERT INTO instruction VALUES('add',':v-Iz',NULL,NULL,NULL,NULL,'primary',0x05,NULL,'8086');
INSERT INTO instruction VALUES('add','Eb-Gb',NULL,NULL,NULL,NULL,'primary',0x00,NULL,'8086');
INSERT INTO instruction VALUES('add','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0x80,0,'8086');
INSERT INTO instruction VALUES('add','Ev-Gv',NULL,NULL,NULL,NULL,'primary',0x01,NULL,'8086');
INSERT INTO instruction VALUES('add','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0x83,0,'8086');
INSERT INTO instruction VALUES('add','Ev-Iz',NULL,NULL,NULL,NULL,'primary',0x81,0,'8086');
INSERT INTO instruction VALUES('add','Gb-Eb',NULL,NULL,NULL,NULL,'primary',0x02,NULL,'8086');
INSERT INTO instruction VALUES('add','Gv-Ev',NULL,NULL,NULL,NULL,'primary',0x03,NULL,'8086');

INSERT INTO instruction VALUES('and',':b-Ib',NULL,NULL,NULL,NULL,'primary',0x24,NULL,'8086');
INSERT INTO instruction VALUES('and',':v-Iz',NULL,NULL,NULL,NULL,'primary',0x25,NULL,'8086');
INSERT INTO instruction VALUES('and','Eb-Gb',NULL,NULL,NULL,NULL,'primary',0x20,NULL,'8086');
INSERT INTO instruction VALUES('and','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0x80,4,'8086');
INSERT INTO instruction VALUES('and','Ev-Gv',NULL,NULL,NULL,NULL,'primary',0x21,NULL,'8086');
INSERT INTO instruction VALUES('and','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0x83,4,'8086');
INSERT INTO instruction VALUES('and','Ev-Iz',NULL,NULL,NULL,NULL,'primary',0x81,4,'8086');
INSERT INTO instruction VALUES('and','Gb-Eb',NULL,NULL,NULL,NULL,'primary',0x22,NULL,'8086');
INSERT INTO instruction VALUES('and','Gv-Ev',NULL,NULL,NULL,NULL,'primary',0x23,NULL,'8086');

INSERT INTO instruction VALUES('arpl','Ew-Gw','legacy',NULL,NULL,NULL,'primary',0x63,NULL,'80286');

INSERT INTO instruction VALUES('bound','Gw-Md','legacy',NULL,'data_size_prefix_16bit',NULL,'primary',0x62,NULL,'80186');
INSERT INTO instruction VALUES('bound','Gd-Mq','legacy',NULL,'data_size_prefix_32bit',NULL,'primary',0x62,NULL,'80186');

INSERT INTO instruction VALUES('bsf','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0xbc,NULL,'80386');

INSERT INTO instruction VALUES('bsr','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0xbd,NULL,'80386');

INSERT INTO instruction VALUES('bswap','=v',NULL,NULL,NULL,NULL,'secondary',0xc8,NULL,'80486');

INSERT INTO instruction VALUES('bt','Ev-Gv',NULL,NULL,NULL,NULL,'secondary',0xa3,NULL,'80386');
INSERT INTO instruction VALUES('bt','Ev-Ib',NULL,NULL,NULL,NULL,'secondary',0xba,4,'80386');

INSERT INTO instruction VALUES('btc','Ev-Gv',NULL,NULL,NULL,NULL,'secondary',0xbb,NULL,'80386');
INSERT INTO instruction VALUES('btc','Ev-Ib',NULL,NULL,NULL,NULL,'secondary',0xba,7,'80386');

INSERT INTO instruction VALUES('btr','Ev-Gv',NULL,NULL,NULL,NULL,'secondary',0xb3,NULL,'80386');
INSERT INTO instruction VALUES('btr','Ev-Ib',NULL,NULL,NULL,NULL,'secondary',0xba,6,'80386');

INSERT INTO instruction VALUES('bts','Ev-Gv',NULL,NULL,NULL,NULL,'secondary',0xab,NULL,'80386');
INSERT INTO instruction VALUES('bts','Ev-Ib',NULL,NULL,NULL,NULL,'secondary',0xba,5,'80386');

INSERT INTO instruction VALUES('call','Ap','legacy',NULL,NULL,NULL,'primary',0x9a,NULL,'8086');
INSERT INTO instruction VALUES('call','Mp',NULL,NULL,NULL,NULL,'primary',0xff,3,'8086');
INSERT INTO instruction VALUES('call','Eq','x86_64',NULL,NULL,NULL,'primary',0xff,2,'8086');
INSERT INTO instruction VALUES('call','Ew','x86_64',NULL,'data_size_prefix_16bit',NULL,'primary',0xff,2,'8086');
INSERT INTO instruction VALUES('call','Ev','legacy',NULL,NULL,NULL,'primary',0xff,2,'8086');
INSERT INTO instruction VALUES('call','Iz',NULL,NULL,NULL,NULL,'primary',0xe8,NULL,'8086');

INSERT INTO instruction VALUES('cbw',NULL,NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0x98,NULL,'8086');
INSERT INTO instruction VALUES('cdq',NULL,NULL,NULL,'data_size_prefix_32bit',NULL,'primary',0x99,NULL,'80386');
INSERT INTO instruction VALUES('cdqe',NULL,'x86_64',NULL,'data_size_prefix_64bit',NULL,'primary',0x98,NULL,'x86_64');

INSERT INTO instruction VALUES('clc',NULL,NULL,NULL,NULL,NULL,'primary',0xf8,NULL,'8086');

INSERT INTO instruction VALUES('cld',NULL,NULL,NULL,NULL,NULL,'primary',0xfc,NULL,'8086');

INSERT INTO instruction VALUES('cli',NULL,NULL,NULL,NULL,NULL,'primary',0xfa,NULL,'8086');

INSERT INTO instruction VALUES('clts',NULL,NULL,NULL,NULL,NULL,'secondary',0x06,NULL,'80286');

INSERT INTO instruction VALUES('cmc',NULL,NULL,NULL,NULL,NULL,'primary',0xf5,NULL,'8086');

INSERT INTO instruction VALUES('cmova','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x47,NULL,'cmov');
INSERT INTO instruction VALUES('cmovae','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x43,NULL,'cmov');
INSERT INTO instruction VALUES('cmovb','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x42,NULL,'cmov');
INSERT INTO instruction VALUES('cmovbe','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x46,NULL,'cmov');
INSERT INTO instruction VALUES('cmove','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x44,NULL,'cmov');
INSERT INTO instruction VALUES('cmovg','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4f,NULL,'cmov');
INSERT INTO instruction VALUES('cmovge','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4d,NULL,'cmov');
INSERT INTO instruction VALUES('cmovl','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4c,NULL,'cmov');
INSERT INTO instruction VALUES('cmovle','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4e,NULL,'cmov');
INSERT INTO instruction VALUES('cmovna','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x46,NULL,'cmov');
INSERT INTO instruction VALUES('cmovnae','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x42,NULL,'cmov');
INSERT INTO instruction VALUES('cmovnb','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x43,NULL,'cmov');
INSERT INTO instruction VALUES('cmovnbe','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x47,NULL,'cmov');
INSERT INTO instruction VALUES('cmovne','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x45,NULL,'cmov');
INSERT INTO instruction VALUES('cmovng','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4e,NULL,'cmov');
INSERT INTO instruction VALUES('cmovnge','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4c,NULL,'cmov');
INSERT INTO instruction VALUES('cmovnl','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4d,NULL,'cmov');
INSERT INTO instruction VALUES('cmovnle','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4f,NULL,'cmov');
INSERT INTO instruction VALUES('cmovno','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x41,NULL,'cmov');
INSERT INTO instruction VALUES('cmovnp','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4b,NULL,'cmov');
INSERT INTO instruction VALUES('cmovns','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x49,NULL,'cmov');
INSERT INTO instruction VALUES('cmovnz','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x45,NULL,'cmov');
INSERT INTO instruction VALUES('cmovo','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x40,NULL,'cmov');
INSERT INTO instruction VALUES('cmovp','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4a,NULL,'cmov');
INSERT INTO instruction VALUES('cmovpe','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4a,NULL,'cmov');
INSERT INTO instruction VALUES('cmovpo','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x4b,NULL,'cmov');
INSERT INTO instruction VALUES('cmovs','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x48,NULL,'cmov');
INSERT INTO instruction VALUES('cmovz','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0x44,NULL,'cmov');

INSERT INTO instruction VALUES('cmp',':b-Ib',NULL,NULL,NULL,NULL,'primary',0x3c,NULL,'8086');
INSERT INTO instruction VALUES('cmp',':v-Iz',NULL,NULL,NULL,NULL,'primary',0x3d,NULL,'8086');
INSERT INTO instruction VALUES('cmp','Eb-Gb',NULL,NULL,NULL,NULL,'primary',0x38,NULL,'8086');
INSERT INTO instruction VALUES('cmp','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0x80,7,'8086');
INSERT INTO instruction VALUES('cmp','Ev-Gv',NULL,NULL,NULL,NULL,'primary',0x39,NULL,'8086');
INSERT INTO instruction VALUES('cmp','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0x83,7,'8086');
INSERT INTO instruction VALUES('cmp','Ev-Iz',NULL,NULL,NULL,NULL,'primary',0x81,7,'8086');
INSERT INTO instruction VALUES('cmp','Gb-Eb',NULL,NULL,NULL,NULL,'primary',0x3a,NULL,'8086');
INSERT INTO instruction VALUES('cmp','Gv-Ev',NULL,NULL,NULL,NULL,'primary',0x3b,NULL,'8086');

INSERT INTO instruction VALUES('cmps','Xb-Yb',NULL,NULL,NULL,NULL,'primary',0xa6,NULL,'8086');
INSERT INTO instruction VALUES('cmps','Xv-Yv',NULL,NULL,NULL,NULL,'primary',0xa7,NULL,'8086');

INSERT INTO instruction VALUES('cmpsb',NULL,NULL,NULL,NULL,NULL,'primary',0xa6,NULL,'8086');
INSERT INTO instruction VALUES('cmpsw',NULL,NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0xa7,NULL,'8086');
INSERT INTO instruction VALUES('cmpsd',NULL,NULL,NULL,'data_size_prefix_32bit',NULL,'primary',0xa7,NULL,'80386');
INSERT INTO instruction VALUES('cmpsq',NULL,'x86_64',NULL,'data_size_prefix_64bit',NULL,'primary',0xa7,NULL,'x86_64');

INSERT INTO instruction VALUES('cmpxchg','Eb-Gb',NULL,NULL,NULL,NULL,'secondary',0xb0,NULL,'80486');
INSERT INTO instruction VALUES('cmpxchg','Ev-Gv',NULL,NULL,NULL,NULL,'secondary',0xb1,NULL,'80486');

INSERT INTO instruction VALUES('cmpxchg8b','Mq',NULL,NULL,NULL,NULL,'secondary',0xc7,1,'cmpxchg8b');
INSERT INTO instruction VALUES('cmpxchg16b','Mo','x86_64',NULL,'data_size_prefix_64bit',NULL,'secondary',0xc7,1,'cmpxchg16b');

INSERT INTO instruction VALUES('cpuid',NULL,NULL,NULL,NULL,NULL,'secondary',0xa2,NULL,'80486');

INSERT INTO instruction VALUES('cqo',NULL,'x86_64',NULL,'data_size_prefix_64bit',NULL,'primary',0x99,NULL,'x86_64');
INSERT INTO instruction VALUES('cwd',NULL,NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0x99,NULL,'8086');
INSERT INTO instruction VALUES('cwde',NULL,NULL,NULL,'data_size_prefix_32bit',NULL,'primary',0x98,NULL,'80386');

INSERT INTO instruction VALUES('daa',NULL,'legacy',NULL,NULL,NULL,'primary',0x27,NULL,'8086');

INSERT INTO instruction VALUES('das',NULL,'legacy',NULL,NULL,NULL,'primary',0x2f,NULL,'8086');

INSERT INTO instruction VALUES('dec','Eb',NULL,NULL,NULL,NULL,'primary',0xfe,1,'8086');
INSERT INTO instruction VALUES('dec','Ev',NULL,NULL,NULL,NULL,'primary',0xff,1,'8086');
INSERT INTO instruction VALUES('dec','=v','legacy',NULL,NULL,NULL,'primary',0x48,NULL,'8086');

INSERT INTO instruction VALUES('div','Eb',NULL,NULL,NULL,NULL,'primary',0xfe,6,'8086');
INSERT INTO instruction VALUES('div','Ev',NULL,NULL,NULL,NULL,'primary',0xff,6,'8086');

INSERT INTO instruction VALUES('emms',NULL,NULL,NULL,NULL,NULL,'secondary',0x77,NULL,'mmx');

INSERT INTO instruction VALUES('enter','Iw-Ib',NULL,NULL,NULL,NULL,'primary',0xc8,NULL,'80186');
INSERT INTO instruction VALUES('enterd','Iw-Ib','legacy',NULL,'data_size_prefix_32bit',NULL,'primary',0xc8,NULL,'80386');
INSERT INTO instruction VALUES('enterq','Iw-Ib','x86_64',NULL,NULL,NULL,'primary',0xc8,NULL,'x86_64');
INSERT INTO instruction VALUES('enterw','Iw-Ib',NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0xc8,NULL,'80186');

INSERT INTO instruction VALUES('f2xm1',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xf0,'8087');

INSERT INTO instruction VALUES('fabs',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xe1,'8087');

INSERT INTO instruction VALUES('fadd','Md',NULL,NULL,NULL,NULL,'primary',0xd8,0,'8087');
INSERT INTO instruction VALUES('fadd','Mq',NULL,NULL,NULL,NULL,'primary',0xdc,0,'8087');
INSERT INTO instruction VALUES('fadd',':7-R7',NULL,NULL,NULL,NULL,'primary',0xd8,0,'8087');
INSERT INTO instruction VALUES('fadd','R7-:7',NULL,NULL,NULL,NULL,'primary',0xdc,0,'8087');
INSERT INTO instruction VALUES('faddp',NULL,NULL,NULL,NULL,NULL,'primary',0xde,0xc1,'8087');
INSERT INTO instruction VALUES('faddp','R7-:7',NULL,NULL,NULL,NULL,'primary',0xde,0,'8087');

INSERT INTO instruction VALUES('fbld','M7',NULL,NULL,NULL,NULL,'primary',0xdf,4,'8087');
INSERT INTO instruction VALUES('fbstp','M7',NULL,NULL,NULL,NULL,'primary',0xdf,6,'8087');

INSERT INTO instruction VALUES('fchs',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xe0,'8087');

INSERT INTO instruction VALUES('fcmovb',':7-R7',NULL,NULL,NULL,NULL,'primary',0xda,0,'cmov&x87');
INSERT INTO instruction VALUES('fcmove',':7-R7',NULL,NULL,NULL,NULL,'primary',0xda,1,'cmov&x87');
INSERT INTO instruction VALUES('fcmovbe',':7-R7',NULL,NULL,NULL,NULL,'primary',0xda,2,'cmov&x87');
INSERT INTO instruction VALUES('fcmovnb',':7-R7',NULL,NULL,NULL,NULL,'primary',0xdb,0,'cmov&x87');
INSERT INTO instruction VALUES('fcmovne',':7-R7',NULL,NULL,NULL,NULL,'primary',0xdb,1,'cmov&x87');
INSERT INTO instruction VALUES('fcmovnbe',':7-R7',NULL,NULL,NULL,NULL,'primary',0xdb,2,'cmov&x87');
INSERT INTO instruction VALUES('fcmovnu',':7-R7',NULL,NULL,NULL,NULL,'primary',0xdb,3,'cmov&x87');
INSERT INTO instruction VALUES('fcmovu',':7-R7',NULL,NULL,NULL,NULL,'primary',0xda,3,'cmov&x87');

INSERT INTO instruction VALUES('fcom',NULL,NULL,NULL,NULL,NULL,'primary',0xd8,0xd1,'8087');
INSERT INTO instruction VALUES('fcom','Md',NULL,NULL,NULL,NULL,'primary',0xd8,2,'8087');
INSERT INTO instruction VALUES('fcom','Mq',NULL,NULL,NULL,NULL,'primary',0xdc,2,'8087');
INSERT INTO instruction VALUES('fcom','R7',NULL,NULL,NULL,NULL,'primary',0xd8,2,'8087');
INSERT INTO instruction VALUES('fcomi',':7-R7',NULL,NULL,NULL,NULL,'primary',0xdb,6,'cmov&x87');
INSERT INTO instruction VALUES('fcomip',':7-R7',NULL,NULL,NULL,NULL,'primary',0xdf,6,'cmov&x87');
INSERT INTO instruction VALUES('fcomp',NULL,NULL,NULL,NULL,NULL,'primary',0xd8,0xd9,'8087');
INSERT INTO instruction VALUES('fcomp','Md',NULL,NULL,NULL,NULL,'primary',0xd8,3,'8087');
INSERT INTO instruction VALUES('fcomp','Mq',NULL,NULL,NULL,NULL,'primary',0xdc,3,'8087');
INSERT INTO instruction VALUES('fcomp','R7',NULL,NULL,NULL,NULL,'primary',0xd8,3,'8087');
INSERT INTO instruction VALUES('fcompp',NULL,NULL,NULL,NULL,NULL,'primary',0xde,0xd9,'8087');

INSERT INTO instruction VALUES('fcos',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xff,'80387');

INSERT INTO instruction VALUES('fdecstp',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xf6,'8087');

INSERT INTO instruction VALUES('fdiv','Md',NULL,NULL,NULL,NULL,'primary',0xd8,6,'8087');
INSERT INTO instruction VALUES('fdiv','Mq',NULL,NULL,NULL,NULL,'primary',0xdc,6,'8087');
INSERT INTO instruction VALUES('fdiv',':7-R7',NULL,NULL,NULL,NULL,'primary',0xd8,6,'8087');
INSERT INTO instruction VALUES('fdiv','R7-:7',NULL,NULL,NULL,NULL,'primary',0xdc,7,'8087');
INSERT INTO instruction VALUES('fdivp',NULL,NULL,NULL,NULL,NULL,'primary',0xde,0xf9,'8087');
INSERT INTO instruction VALUES('fdivp','R7-:7',NULL,NULL,NULL,NULL,'primary',0xde,7,'8087');

INSERT INTO instruction VALUES('fdivr','Md',NULL,NULL,NULL,NULL,'primary',0xd8,7,'8087');
INSERT INTO instruction VALUES('fdivr','Mq',NULL,NULL,NULL,NULL,'primary',0xdc,7,'8087');
INSERT INTO instruction VALUES('fdivr',':7-R7',NULL,NULL,NULL,NULL,'primary',0xd8,7,'8087');
INSERT INTO instruction VALUES('fdivr','R7-:7',NULL,NULL,NULL,NULL,'primary',0xdc,6,'8087');
INSERT INTO instruction VALUES('fdivrp',NULL,NULL,NULL,NULL,NULL,'primary',0xde,0xf1,'8087');
INSERT INTO instruction VALUES('fdivrp','R7-:7',NULL,NULL,NULL,NULL,'primary',0xde,6,'8087');

INSERT INTO instruction VALUES('ffree','R7',NULL,NULL,NULL,NULL,'primary',0xdd,0,'8087');

INSERT INTO instruction VALUES('ffreep','R7',NULL,NULL,NULL,NULL,'primary',0xdf,0,'8087');

INSERT INTO instruction VALUES('fiadd','Md',NULL,NULL,NULL,NULL,'primary',0xda,0,'8087');
INSERT INTO instruction VALUES('fiadd','Mw',NULL,NULL,NULL,NULL,'primary',0xde,0,'8087');

INSERT INTO instruction VALUES('ficom','Md',NULL,NULL,NULL,NULL,'primary',0xda,2,'8087');
INSERT INTO instruction VALUES('ficom','Mw',NULL,NULL,NULL,NULL,'primary',0xde,2,'8087');
INSERT INTO instruction VALUES('ficomp','Md',NULL,NULL,NULL,NULL,'primary',0xda,3,'8087');
INSERT INTO instruction VALUES('ficomp','Mw',NULL,NULL,NULL,NULL,'primary',0xde,3,'8087');

INSERT INTO instruction VALUES('fidiv','Md',NULL,NULL,NULL,NULL,'primary',0xda,6,'8087');
INSERT INTO instruction VALUES('fidiv','Mw',NULL,NULL,NULL,NULL,'primary',0xde,6,'8087');

INSERT INTO instruction VALUES('fidivr','Md',NULL,NULL,NULL,NULL,'primary',0xda,7,'8087');
INSERT INTO instruction VALUES('fidivr','Mw',NULL,NULL,NULL,NULL,'primary',0xde,7,'8087');

INSERT INTO instruction VALUES('fild','Md',NULL,NULL,NULL,NULL,'primary',0xdb,0,'8087');
INSERT INTO instruction VALUES('fild','Mq',NULL,NULL,NULL,NULL,'primary',0xdf,5,'8087');
INSERT INTO instruction VALUES('fild','Mw',NULL,NULL,NULL,NULL,'primary',0xdf,0,'8087');

INSERT INTO instruction VALUES('fisttp','Md',NULL,NULL,NULL,NULL,'primary',0xdb,1,'sse3&x87');
INSERT INTO instruction VALUES('fisttp','Mq',NULL,NULL,NULL,NULL,'primary',0xdd,1,'sse3&x87');
INSERT INTO instruction VALUES('fisttp','Mw',NULL,NULL,NULL,NULL,'primary',0xdf,1,'sse3&x87');

INSERT INTO instruction VALUES('fiadd','Md',NULL,NULL,NULL,NULL,'primary',0xda,0,'8087');
INSERT INTO instruction VALUES('fiadd','Mw',NULL,NULL,NULL,NULL,'primary',0xde,0,'8087');

INSERT INTO instruction VALUES('fimil','Md',NULL,NULL,NULL,NULL,'primary',0xda,1,'8087');
INSERT INTO instruction VALUES('fimul','Mw',NULL,NULL,NULL,NULL,'primary',0xde,1,'8087');

INSERT INTO instruction VALUES('fist','Md',NULL,NULL,NULL,NULL,'primary',0xdb,2,'8087');
INSERT INTO instruction VALUES('fist','Mw',NULL,NULL,NULL,NULL,'primary',0xdf,2,'8087');

INSERT INTO instruction VALUES('fistp','Md',NULL,NULL,NULL,NULL,'primary',0xdb,3,'8087');
INSERT INTO instruction VALUES('fistp','Mq',NULL,NULL,NULL,NULL,'primary',0xdf,7,'8087');
INSERT INTO instruction VALUES('fistp','Mw',NULL,NULL,NULL,NULL,'primary',0xdf,3,'8087');

INSERT INTO instruction VALUES('fisub','Md',NULL,NULL,NULL,NULL,'primary',0xda,4,'8087');
INSERT INTO instruction VALUES('fisub','Mw',NULL,NULL,NULL,NULL,'primary',0xde,4,'8087');

INSERT INTO instruction VALUES('fisubr','Md',NULL,NULL,NULL,NULL,'primary',0xda,5,'8087');
INSERT INTO instruction VALUES('fisubr','Mw',NULL,NULL,NULL,NULL,'primary',0xde,5,'8087');

INSERT INTO instruction VALUES('fld','Md',NULL,NULL,NULL,NULL,'primary',0xd9,0,'8087');
INSERT INTO instruction VALUES('fld','Mq',NULL,NULL,NULL,NULL,'primary',0xdd,0,'8087');
INSERT INTO instruction VALUES('fld','M7',NULL,NULL,NULL,NULL,'primary',0xdb,5,'8087');
INSERT INTO instruction VALUES('fld','R7',NULL,NULL,NULL,NULL,'primary',0xd9,0,'8087');

INSERT INTO instruction VALUES('fld1',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xe8,'8087');
INSERT INTO instruction VALUES('fldl2e',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xea,'8087');
INSERT INTO instruction VALUES('fldl2t',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xe9,'8087');
INSERT INTO instruction VALUES('fldlg2',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xec,'8087');
INSERT INTO instruction VALUES('fldln2',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xed,'8087');
INSERT INTO instruction VALUES('fldpi',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xeb,'8087');
INSERT INTO instruction VALUES('fldz',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xee,'8087');

INSERT INTO instruction VALUES('fldcw','Mw',NULL,NULL,NULL,NULL,'primary',0xd9,5,'8087');

INSERT INTO instruction VALUES('fldenv','M112/224',NULL,NULL,NULL,NULL,'primary',0xd9,4,'8087');

INSERT INTO instruction VALUES('fmul','Md',NULL,NULL,NULL,NULL,'primary',0xd8,1,'8087');
INSERT INTO instruction VALUES('fmul','Mq',NULL,NULL,NULL,NULL,'primary',0xdc,1,'8087');
INSERT INTO instruction VALUES('fmul',':7-R7',NULL,NULL,NULL,NULL,'primary',0xd8,1,'8087');
INSERT INTO instruction VALUES('fmul','R7-:7',NULL,NULL,NULL,NULL,'primary',0xdc,1,'8087');
INSERT INTO instruction VALUES('fmulp','R7-:7',NULL,NULL,NULL,NULL,'primary',0xde,1,'8087');

INSERT INTO instruction VALUES('fnclex',NULL,NULL,NULL,NULL,NULL,'primary',0xdb,0xe2,'8087');

INSERT INTO instruction VALUES('fninit',NULL,NULL,NULL,NULL,NULL,'primary',0xdb,0xe3,'8087');

INSERT INTO instruction VALUES('fndisi',NULL,NULL,NULL,NULL,NULL,'primary',0xdb,0xe1,'8087');

INSERT INTO instruction VALUES('fneni',NULL,NULL,NULL,NULL,NULL,'primary',0xdb,0xe0,'8087');

INSERT INTO instruction VALUES('fnop',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xd0,'8087');

INSERT INTO instruction VALUES('fnsave','M752/864',NULL,NULL,NULL,NULL,'primary',0xdd,6,'8087');

INSERT INTO instruction VALUES('fnsetpm',NULL,NULL,NULL,NULL,NULL,'primary',0xdb,0xe4,'80287');

INSERT INTO instruction VALUES('fnstcw','Mw',NULL,NULL,NULL,NULL,'primary',0xd9,7,'8087');

INSERT INTO instruction VALUES('fnstenv','M112/224',NULL,NULL,NULL,NULL,'primary',0xd9,6,'8087');

INSERT INTO instruction VALUES('fnstsw',':w',NULL,NULL,NULL,NULL,'primary',0xdf,0xe0,'80287');
INSERT INTO instruction VALUES('fnstsw','Mw',NULL,NULL,NULL,NULL,'primary',0xdd,7,'8087');

INSERT INTO instruction VALUES('fpatan',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xf3,'8087');

INSERT INTO instruction VALUES('fprem',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xf8,'8087');
INSERT INTO instruction VALUES('fprem1',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xf5,'80387');

INSERT INTO instruction VALUES('fptan',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xf2,'8087');

INSERT INTO instruction VALUES('frndint',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xfc,'8087');

INSERT INTO instruction VALUES('frstor','M752/864',NULL,NULL,NULL,NULL,'primary',0xdd,4,'8087');

INSERT INTO instruction VALUES('frstpm',NULL,NULL,NULL,NULL,NULL,'primary',0xd5,0x02,'80287');

INSERT INTO instruction VALUES('fscale',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xfd,'8087');

INSERT INTO instruction VALUES('fsin',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xfe,'80387');

INSERT INTO instruction VALUES('fsincos',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xfb,'80387');

INSERT INTO instruction VALUES('fsqrt',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xfa,'8087');

INSERT INTO instruction VALUES('fst','Md',NULL,NULL,NULL,NULL,'primary',0xd9,2,'8087');
INSERT INTO instruction VALUES('fst','Mq',NULL,NULL,NULL,NULL,'primary',0xdd,2,'8087');
INSERT INTO instruction VALUES('fst','R7',NULL,NULL,NULL,NULL,'primary',0xdd,2,'8087');
INSERT INTO instruction VALUES('fstp','Md',NULL,NULL,NULL,NULL,'primary',0xd9,3,'8087');
INSERT INTO instruction VALUES('fstp','Mq',NULL,NULL,NULL,NULL,'primary',0xdd,3,'8087');
INSERT INTO instruction VALUES('fstp','Md',NULL,NULL,NULL,NULL,'primary',0xdb,7,'8087');
INSERT INTO instruction VALUES('fstp','R7',NULL,NULL,NULL,NULL,'primary',0xdd,3,'8087');
INSERT INTO instruction VALUES('fstpnce','R7',NULL,NULL,NULL,NULL,'primary',0xd9,3,'8087');

INSERT INTO instruction VALUES('fsub','Md',NULL,NULL,NULL,NULL,'primary',0xd8,4,'8087');
INSERT INTO instruction VALUES('fsub','Mq',NULL,NULL,NULL,NULL,'primary',0xdc,4,'8087');
INSERT INTO instruction VALUES('fsub',':7-R7',NULL,NULL,NULL,NULL,'primary',0xd8,4,'8087');
INSERT INTO instruction VALUES('fsub','R7-:7',NULL,NULL,NULL,NULL,'primary',0xdc,5,'8087');
INSERT INTO instruction VALUES('fsubp',NULL,NULL,NULL,NULL,NULL,'primary',0xde,0xe9,'8087');
INSERT INTO instruction VALUES('fsubp','R7-:7',NULL,NULL,NULL,NULL,'primary',0xde,5,'8087');

INSERT INTO instruction VALUES('fsubr','Md',NULL,NULL,NULL,NULL,'primary',0xd8,5,'8087');
INSERT INTO instruction VALUES('fsubr','Mq',NULL,NULL,NULL,NULL,'primary',0xdc,5,'8087');
INSERT INTO instruction VALUES('fsubr',':7-R7',NULL,NULL,NULL,NULL,'primary',0xd8,5,'8087');
INSERT INTO instruction VALUES('fsubr','R7-:7',NULL,NULL,NULL,NULL,'primary',0xdc,4,'8087');
INSERT INTO instruction VALUES('fsubrp',NULL,NULL,NULL,NULL,NULL,'primary',0xde,0xe1,'8087');
INSERT INTO instruction VALUES('fsubrp','R7-:7',NULL,NULL,NULL,NULL,'primary',0xde,4,'8087');

INSERT INTO instruction VALUES('ftst',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xe4,'8087');

INSERT INTO instruction VALUES('fucom',NULL,NULL,NULL,NULL,NULL,'primary',0xdd,0xe1,'80387');
INSERT INTO instruction VALUES('fucom','R7',NULL,NULL,NULL,NULL,'primary',0xdd,4,'80387');
INSERT INTO instruction VALUES('fucomi',':7-R7',NULL,NULL,NULL,NULL,'primary',0xdb,5,'cmov&x87');
INSERT INTO instruction VALUES('fucomip',':7-R7',NULL,NULL,NULL,NULL,'primary',0xdf,5,'cmov&x87');
INSERT INTO instruction VALUES('fucomp',NULL,NULL,NULL,NULL,NULL,'primary',0xdd,0xe9,'80387');
INSERT INTO instruction VALUES('fucomp','R7',NULL,NULL,NULL,NULL,'primary',0xdd,5,'80387');
INSERT INTO instruction VALUES('fucompp',NULL,NULL,NULL,NULL,NULL,'primary',0xda,0xe9,'80387');

INSERT INTO instruction VALUES('fwait',NULL,NULL,NULL,NULL,NULL,'primary',0x9b,NULL,'8086');

INSERT INTO instruction VALUES('fxam',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xe5,'8087');

INSERT INTO instruction VALUES('fldenv','M112/224',NULL,NULL,NULL,NULL,'primary',0xd9,4,'8087');

INSERT INTO instruction VALUES('fxch',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xc9,'8087');
INSERT INTO instruction VALUES('fxch','R7',NULL,NULL,NULL,NULL,'primary',0xd9,1,'8087');

INSERT INTO instruction VALUES('fxrstor','M512',NULL,NULL,NULL,NULL,'secondary',0xae,1,'fxsr');
INSERT INTO instruction VALUES('fxrstor64','M512','x86_64',NULL,'data_size_prefix_64bit',NULL,'secondary',0xae,1,'fxsr');

INSERT INTO instruction VALUES('fxsave','M512',NULL,NULL,NULL,NULL,'secondary',0xae,0,'fxsr');
INSERT INTO instruction VALUES('fxsave64','M512','x86_64',NULL,'data_size_prefix_64bit',NULL,'secondary',0xae,0,'fxsr');

INSERT INTO instruction VALUES('fxtract',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xf4,'8087');

INSERT INTO instruction VALUES('fyl2x',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xf1,'8087');

INSERT INTO instruction VALUES('fyl2xp1',NULL,NULL,NULL,NULL,NULL,'primary',0xd9,0xf9,'8087');

INSERT INTO instruction VALUES('hlt',NULL,'legacy',NULL,NULL,NULL,'primary',0xf4,NULL,'8086');

INSERT INTO instruction VALUES('ibts','Ev-Gv',NULL,NULL,NULL,NULL,'secondary',0xa7,NULL,'80386');
INSERT INTO instruction VALUES('ibts','Ev-:v-#b-Gv',NULL,NULL,NULL,NULL,'secondary',0xa7,NULL,'80386');

INSERT INTO instruction VALUES('idiv','Eb',NULL,NULL,NULL,NULL,'primary',0xfe,7,'8086');
INSERT INTO instruction VALUES('idiv','Ev',NULL,NULL,NULL,NULL,'primary',0xff,7,'8086');

INSERT INTO instruction VALUES('imul','Eb',NULL,NULL,NULL,NULL,'primary',0xfe,5,'8086');
INSERT INTO instruction VALUES('imul','Ev',NULL,NULL,NULL,NULL,'primary',0xff,5,'8086');
INSERT INTO instruction VALUES('imul','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0xaf,NULL,'80386');
INSERT INTO instruction VALUES('imul','Gv-Ev-Ib',NULL,NULL,NULL,NULL,'primary',0x6b,NULL,'80186');
INSERT INTO instruction VALUES('imul','Gv-Ev-Iz',NULL,NULL,NULL,NULL,'primary',0x69,NULL,'80186');

INSERT INTO instruction VALUES('in',':b-Ib',NULL,NULL,NULL,NULL,'primary',0xe4,NULL,'8086');
INSERT INTO instruction VALUES('in',':z-Ib',NULL,NULL,NULL,NULL,'primary',0xe5,NULL,'8086');
INSERT INTO instruction VALUES('in',':b-&w',NULL,NULL,NULL,NULL,'primary',0xec,NULL,'8086');
INSERT INTO instruction VALUES('in',':z-&w',NULL,NULL,NULL,NULL,'primary',0xed,NULL,'8086');

INSERT INTO instruction VALUES('inc','Eb',NULL,NULL,NULL,NULL,'primary',0xfe,0,'8086');
INSERT INTO instruction VALUES('inc','Ev',NULL,NULL,NULL,NULL,'primary',0xff,0,'8086');
INSERT INTO instruction VALUES('inc','=v','legacy',NULL,NULL,NULL,'primary',0x40,NULL,'8086');

INSERT INTO instruction VALUES('ins','Yb-&w',NULL,NULL,NULL,NULL,'primary',0x6c,NULL,'80186');
INSERT INTO instruction VALUES('ins','Yz-&w',NULL,NULL,NULL,NULL,'primary',0x6d,NULL,'80186');

INSERT INTO instruction VALUES('insb',NULL,NULL,NULL,NULL,NULL,'primary',0x6c,NULL,'80186');
INSERT INTO instruction VALUES('insw',NULL,NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0x6d,NULL,'80186');
INSERT INTO instruction VALUES('insd',NULL,NULL,NULL,'data_size_prefix_32bit',NULL,'primary',0x6d,NULL,'80386');

INSERT INTO instruction VALUES('int','Ib',NULL,NULL,NULL,NULL,'primary',0xcd,NULL,'8086');
INSERT INTO instruction VALUES('int1',NULL,NULL,NULL,NULL,NULL,'primary',0xf1,NULL,'8086');
INSERT INTO instruction VALUES('int3',NULL,NULL,NULL,NULL,NULL,'primary',0xcc,NULL,'8086');
INSERT INTO instruction VALUES('into',NULL,'legacy',NULL,NULL,NULL,'primary',0xce,NULL,'8086');

INSERT INTO instruction VALUES('invd',NULL,NULL,NULL,NULL,NULL,'secondary',0x08,NULL,'80486');

INSERT INTO instruction VALUES('invlpg','M',NULL,NULL,NULL,NULL,'secondary',0x01,7,'80486');

INSERT INTO instruction VALUES('iret',NULL,NULL,NULL,NULL,NULL,'primary',0xcf,NULL,'8086');
INSERT INTO instruction VALUES('iretd',NULL,NULL,NULL,'data_size_prefix_32bit',NULL,'primary',0xcf,NULL,'80386');
INSERT INTO instruction VALUES('iretq',NULL,'x86_64',NULL,'data_size_prefix_64bit',NULL,'primary',0xcf,NULL,'x86_64');
INSERT INTO instruction VALUES('iretw',NULL,NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0xcf,NULL,'8086');

INSERT INTO instruction VALUES('ja','Ib',NULL,NULL,NULL,NULL,'primary',0x77,NULL,'8086');
INSERT INTO instruction VALUES('ja','Iz',NULL,NULL,NULL,NULL,'secondary',0x87,NULL,'80386');
INSERT INTO instruction VALUES('jae','Ib',NULL,NULL,NULL,NULL,'primary',0x73,NULL,'8086');
INSERT INTO instruction VALUES('jae','Iz',NULL,NULL,NULL,NULL,'secondary',0x83,NULL,'80386');
INSERT INTO instruction VALUES('jb','Ib',NULL,NULL,NULL,NULL,'primary',0x72,NULL,'8086');
INSERT INTO instruction VALUES('jb','Iz',NULL,NULL,NULL,NULL,'secondary',0x82,NULL,'80386');
INSERT INTO instruction VALUES('jbe','Ib',NULL,NULL,NULL,NULL,'primary',0x76,NULL,'8086');
INSERT INTO instruction VALUES('jbe','Iz',NULL,NULL,NULL,NULL,'secondary',0x86,NULL,'80386');
INSERT INTO instruction VALUES('je','Ib',NULL,NULL,NULL,NULL,'primary',0x74,NULL,'8086');
INSERT INTO instruction VALUES('je','Iz',NULL,NULL,NULL,NULL,'secondary',0x84,NULL,'80386');
INSERT INTO instruction VALUES('jg','Ib',NULL,NULL,NULL,NULL,'primary',0x7f,NULL,'8086');
INSERT INTO instruction VALUES('jg','Iz',NULL,NULL,NULL,NULL,'secondary',0x8f,NULL,'80386');
INSERT INTO instruction VALUES('jge','Ib',NULL,NULL,NULL,NULL,'primary',0x7d,NULL,'8086');
INSERT INTO instruction VALUES('jge','Iz',NULL,NULL,NULL,NULL,'secondary',0x8d,NULL,'80386');
INSERT INTO instruction VALUES('jl','Ib',NULL,NULL,NULL,NULL,'primary',0x7c,NULL,'8086');
INSERT INTO instruction VALUES('jl','Iz',NULL,NULL,NULL,NULL,'secondary',0x8c,NULL,'80386');
INSERT INTO instruction VALUES('jle','Ib',NULL,NULL,NULL,NULL,'primary',0x7e,NULL,'8086');
INSERT INTO instruction VALUES('jle','Iz',NULL,NULL,NULL,NULL,'secondary',0x8e,NULL,'80386');
INSERT INTO instruction VALUES('jna','Ib',NULL,NULL,NULL,NULL,'primary',0x76,NULL,'8086');
INSERT INTO instruction VALUES('jna','Iz',NULL,NULL,NULL,NULL,'secondary',0x86,NULL,'80386');
INSERT INTO instruction VALUES('jnae','Ib',NULL,NULL,NULL,NULL,'primary',0x72,NULL,'8086');
INSERT INTO instruction VALUES('jnae','Iz',NULL,NULL,NULL,NULL,'secondary',0x82,NULL,'80386');
INSERT INTO instruction VALUES('jnb','Ib',NULL,NULL,NULL,NULL,'primary',0x73,NULL,'8086');
INSERT INTO instruction VALUES('jnb','Iz',NULL,NULL,NULL,NULL,'secondary',0x83,NULL,'80386');
INSERT INTO instruction VALUES('jnbe','Ib',NULL,NULL,NULL,NULL,'primary',0x77,NULL,'8086');
INSERT INTO instruction VALUES('jnbe','Iz',NULL,NULL,NULL,NULL,'secondary',0x87,NULL,'80386');
INSERT INTO instruction VALUES('jne','Ib',NULL,NULL,NULL,NULL,'primary',0x75,NULL,'8086');
INSERT INTO instruction VALUES('jne','Iz',NULL,NULL,NULL,NULL,'secondary',0x85,NULL,'80386');
INSERT INTO instruction VALUES('jng','Ib',NULL,NULL,NULL,NULL,'primary',0x7e,NULL,'8086');
INSERT INTO instruction VALUES('jng','Iz',NULL,NULL,NULL,NULL,'secondary',0x8e,NULL,'80386');
INSERT INTO instruction VALUES('jnge','Ib',NULL,NULL,NULL,NULL,'primary',0x7c,NULL,'8086');
INSERT INTO instruction VALUES('jnge','Iz',NULL,NULL,NULL,NULL,'secondary',0x8c,NULL,'80386');
INSERT INTO instruction VALUES('jnl','Ib',NULL,NULL,NULL,NULL,'primary',0x7d,NULL,'8086');
INSERT INTO instruction VALUES('jnl','Iz',NULL,NULL,NULL,NULL,'secondary',0x8d,NULL,'80386');
INSERT INTO instruction VALUES('jnle','Ib',NULL,NULL,NULL,NULL,'primary',0x7f,NULL,'8086');
INSERT INTO instruction VALUES('jnle','Iz',NULL,NULL,NULL,NULL,'secondary',0x8f,NULL,'80386');
INSERT INTO instruction VALUES('jno','Ib',NULL,NULL,NULL,NULL,'primary',0x71,NULL,'8086');
INSERT INTO instruction VALUES('jno','Iz',NULL,NULL,NULL,NULL,'secondary',0x81,NULL,'80386');
INSERT INTO instruction VALUES('jnp','Ib',NULL,NULL,NULL,NULL,'primary',0x7b,NULL,'8086');
INSERT INTO instruction VALUES('jnp','Iz',NULL,NULL,NULL,NULL,'secondary',0x8b,NULL,'80386');
INSERT INTO instruction VALUES('jns','Ib',NULL,NULL,NULL,NULL,'primary',0x79,NULL,'8086');
INSERT INTO instruction VALUES('jns','Iz',NULL,NULL,NULL,NULL,'secondary',0x89,NULL,'80386');
INSERT INTO instruction VALUES('jnz','Ib',NULL,NULL,NULL,NULL,'primary',0x75,NULL,'8086');
INSERT INTO instruction VALUES('jnz','Iz',NULL,NULL,NULL,NULL,'secondary',0x85,NULL,'80386');
INSERT INTO instruction VALUES('jo','Ib',NULL,NULL,NULL,NULL,'primary',0x70,NULL,'8086');
INSERT INTO instruction VALUES('jo','Iz',NULL,NULL,NULL,NULL,'secondary',0x80,NULL,'80386');
INSERT INTO instruction VALUES('jp','Ib',NULL,NULL,NULL,NULL,'primary',0x7a,NULL,'8086');
INSERT INTO instruction VALUES('jp','Iz',NULL,NULL,NULL,NULL,'secondary',0x8a,NULL,'80386');
INSERT INTO instruction VALUES('jpe','Ib',NULL,NULL,NULL,NULL,'primary',0x7a,NULL,'8086');
INSERT INTO instruction VALUES('jpe','Iz',NULL,NULL,NULL,NULL,'secondary',0x8a,NULL,'80386');
INSERT INTO instruction VALUES('jpo','Ib',NULL,NULL,NULL,NULL,'primary',0x7b,NULL,'8086');
INSERT INTO instruction VALUES('jpo','Iz',NULL,NULL,NULL,NULL,'secondary',0x8b,NULL,'80386');
INSERT INTO instruction VALUES('js','Ib',NULL,NULL,NULL,NULL,'primary',0x78,NULL,'8086');
INSERT INTO instruction VALUES('js','Iz',NULL,NULL,NULL,NULL,'secondary',0x88,NULL,'80386');
INSERT INTO instruction VALUES('jz','Ib',NULL,NULL,NULL,NULL,'primary',0x74,NULL,'8086');
INSERT INTO instruction VALUES('jz','Iz',NULL,NULL,NULL,NULL,'secondary',0x84,NULL,'80386');

INSERT INTO instruction VALUES('jcxz','Ib','legacy','address_size_prefix_16bit',NULL,NULL,'primary',0xe3,NULL,'8086');
INSERT INTO instruction VALUES('jecxz','Ib',NULL,'address_size_prefix_32bit',NULL,NULL,'primary',0xe3,NULL,'8086');
INSERT INTO instruction VALUES('jrcxz','Ib','x86_64','address_size_prefix_64bit',NULL,NULL,'primary',0xe3,NULL,'x86_64');

INSERT INTO instruction VALUES('jmp','Ap','legacy',NULL,NULL,NULL,'primary',0xea,NULL,'8086');
INSERT INTO instruction VALUES('jmp','Mp',NULL,NULL,NULL,NULL,'primary',0xff,5,'8086');
INSERT INTO instruction VALUES('jmp','Eq','x86_64',NULL,NULL,NULL,'primary',0xff,4,'8086');
INSERT INTO instruction VALUES('jmp','Ew','x86_64',NULL,'data_size_prefix_16bit',NULL,'primary',0xff,4,'8086');
INSERT INTO instruction VALUES('jmp','Ev','legacy',NULL,NULL,NULL,'primary',0xff,4,'8086');
INSERT INTO instruction VALUES('jmp','Ib',NULL,NULL,NULL,NULL,'primary',0xeb,NULL,'8086');
INSERT INTO instruction VALUES('jmp','Iz',NULL,NULL,NULL,NULL,'primary',0xe9,NULL,'8086');

INSERT INTO instruction VALUES('lahf',NULL,NULL,NULL,NULL,NULL,'primary',0x9f,NULL,'lahf');

INSERT INTO instruction VALUES('lar','Gz-Ew',NULL,NULL,NULL,NULL,'secondary',0x02,NULL,'80286');

INSERT INTO instruction VALUES('lds','Gv-Mp','legacy',NULL,NULL,NULL,'primary',0xc5,NULL,'8086');

INSERT INTO instruction VALUES('lea','Gv-M',NULL,NULL,NULL,NULL,'primary',0x8d,NULL,'8086');

INSERT INTO instruction VALUES('leave',NULL,NULL,NULL,NULL,NULL,'primary',0xc8,NULL,'80186');
INSERT INTO instruction VALUES('leaved',NULL,'legacy',NULL,'data_size_prefix_32bit',NULL,'primary',0xc8,NULL,'80386');
INSERT INTO instruction VALUES('leaveq',NULL,'x86_64',NULL,NULL,NULL,'primary',0xc8,NULL,'x86_64');
INSERT INTO instruction VALUES('leavew',NULL,NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0xc8,NULL,'80186');

INSERT INTO instruction VALUES('les','Gv-Mp','legacy',NULL,NULL,NULL,'primary',0xc4,NULL,'8086');

INSERT INTO instruction VALUES('lfs','Gv-Mp',NULL,NULL,NULL,NULL,'secondary',0xb4,NULL,'80386');

INSERT INTO instruction VALUES('lgdt','Ms',NULL,NULL,NULL,NULL,'secondary',0x01,2,'80286');

INSERT INTO instruction VALUES('lgs','Gv-Mp',NULL,NULL,NULL,NULL,'secondary',0xb5,NULL,'80386');

INSERT INTO instruction VALUES('lidt','Ms',NULL,NULL,NULL,NULL,'secondary',0x01,3,'80286');

INSERT INTO instruction VALUES('lldt','Ew',NULL,NULL,NULL,NULL,'secondary',0x00,2,'80286');

INSERT INTO instruction VALUES('lmsw','Ew',NULL,NULL,NULL,NULL,'secondary',0x01,6,'80286');

INSERT INTO instruction VALUES('lods',':b-Xb',NULL,NULL,NULL,NULL,'primary',0xac,NULL,'8086');
INSERT INTO instruction VALUES('lods','Xb',NULL,NULL,NULL,NULL,'primary',0xac,NULL,'8086');
INSERT INTO instruction VALUES('lods',':v-Xv',NULL,NULL,NULL,NULL,'primary',0xad,NULL,'8086');
INSERT INTO instruction VALUES('lods','Xv',NULL,NULL,NULL,NULL,'primary',0xad,NULL,'8086');

INSERT INTO instruction VALUES('lodsb',NULL,NULL,NULL,NULL,NULL,'primary',0xac,NULL,'8086');
INSERT INTO instruction VALUES('lodsw',NULL,NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0xad,NULL,'8086');
INSERT INTO instruction VALUES('lodsd',NULL,NULL,NULL,'data_size_prefix_32bit',NULL,'primary',0xad,NULL,'80386');
INSERT INTO instruction VALUES('lodsq',NULL,'x86_64',NULL,'data_size_prefix_64bit',NULL,'primary',0xad,NULL,'x86_64');

INSERT INTO instruction VALUES('loop','Ib',NULL,NULL,NULL,NULL,'primary',0xe2,NULL,'8086');
INSERT INTO instruction VALUES('loopd','Ib',NULL,'address_size_prefix_32bit',NULL,NULL,'primary',0xe2,NULL,'80386');
INSERT INTO instruction VALUES('loope','Ib',NULL,NULL,NULL,NULL,'primary',0xe1,NULL,'8086');
INSERT INTO instruction VALUES('looped','Ib',NULL,'address_size_prefix_32bit',NULL,NULL,'primary',0xe1,NULL,'80386');
INSERT INTO instruction VALUES('loopeq','Ib','x86_64','address_size_prefix_64bit',NULL,NULL,'primary',0xe1,NULL,'x86_64');
INSERT INTO instruction VALUES('loopne','Ib',NULL,NULL,NULL,NULL,'primary',0xe0,NULL,'8086');
INSERT INTO instruction VALUES('loopned','Ib',NULL,'address_size_prefix_32bit',NULL,NULL,'primary',0xe0,NULL,'80386');
INSERT INTO instruction VALUES('loopneq','Ib','x86_64','address_size_prefix_64bit',NULL,NULL,'primary',0xe0,NULL,'x86_64');
INSERT INTO instruction VALUES('loopq','Ib','x86_64','address_size_prefix_64bit',NULL,NULL,'primary',0xe2,NULL,'x86_64');
INSERT INTO instruction VALUES('loopw','Ib','legacy','address_size_prefix_16bit',NULL,NULL,'primary',0xe2,NULL,'8086');
INSERT INTO instruction VALUES('loopew','Ib','legacy','address_size_prefix_16bit',NULL,NULL,'primary',0xe1,NULL,'8086');
INSERT INTO instruction VALUES('loopnew','Ib','legacy','address_size_prefix_16bit',NULL,NULL,'primary',0xe0,NULL,'8086');

INSERT INTO instruction VALUES('lsl','Gv-Mw',NULL,NULL,NULL,NULL,'secondary',0x03,NULL,'8086');
INSERT INTO instruction VALUES('lsl','Gd-Rd',NULL,NULL,'data_size_prefix_32bit',NULL,'secondary',0x03,NULL,'8086');
INSERT INTO instruction VALUES('lsl','Gq-Rd','x86_64',NULL,'data_size_prefix_64bit',NULL,'secondary',0x03,NULL,'8086');
INSERT INTO instruction VALUES('lsl','Gw-Rw',NULL,NULL,'data_size_prefix_16bit',NULL,'secondary',0x03,NULL,'8086');

INSERT INTO instruction VALUES('lss','Gv-Mp','legacy',NULL,NULL,NULL,'secondary',0xb2,NULL,'80386');

INSERT INTO instruction VALUES('ltr','Ew',NULL,NULL,NULL,NULL,'secondary',0x00,3,'80286');

INSERT INTO instruction VALUES('lzcnt','Gv-Ev',NULL,NULL,NULL,'0xf3','secondary',0xbc,NULL,'lzcnt');

INSERT INTO instruction VALUES('mov',':b-Ob',NULL,NULL,NULL,NULL,'primary',0xa0,NULL,'8086');
INSERT INTO instruction VALUES('mov',':v-Ov',NULL,NULL,NULL,NULL,'primary',0xa1,NULL,'8086');
INSERT INTO instruction VALUES('mov','=b-Ib',NULL,NULL,NULL,NULL,'primary',0xb0,NULL,'8086');
INSERT INTO instruction VALUES('mov','=v-Iv',NULL,NULL,NULL,NULL,'primary',0xb8,NULL,'8086');
INSERT INTO instruction VALUES('mov','Cd-Rd',NULL,NULL,NULL,NULL,'secondary',0x22,NULL,'80386');
INSERT INTO instruction VALUES('mov','Cq-Rq','x86_64',NULL,NULL,NULL,'secondary',0x22,NULL,'x86_64');
INSERT INTO instruction VALUES('mov','Dd-Rd',NULL,NULL,NULL,NULL,'secondary',0x23,NULL,'80386');
INSERT INTO instruction VALUES('mov','Dq-Rq','x86_64',NULL,NULL,NULL,'secondary',0x23,NULL,'x86_64');
INSERT INTO instruction VALUES('mov','Eb-Gb',NULL,NULL,NULL,NULL,'primary',0x88,NULL,'8086');
INSERT INTO instruction VALUES('mov','Ev-Iz',NULL,NULL,NULL,NULL,'primary',0xc7,0,'8086');
INSERT INTO instruction VALUES('mov','Ev-Gv',NULL,NULL,NULL,NULL,'primary',0x89,NULL,'8086');
INSERT INTO instruction VALUES('mov','Ev-Sw',NULL,NULL,NULL,NULL,'primary',0x8c,NULL,'8086');
INSERT INTO instruction VALUES('mov','Gb-Eb',NULL,NULL,NULL,NULL,'primary',0x8a,NULL,'8086');
INSERT INTO instruction VALUES('mov','Gv-Ev',NULL,NULL,NULL,NULL,'primary',0x8b,NULL,'8086');
INSERT INTO instruction VALUES('mov','Rd-Cd',NULL,NULL,NULL,NULL,'secondary',0x20,NULL,'80386');
INSERT INTO instruction VALUES('mov','Rq-Cq','x86_64',NULL,NULL,NULL,'secondary',0x20,NULL,'x86_64');
INSERT INTO instruction VALUES('mov','Rd-Dd',NULL,NULL,NULL,NULL,'secondary',0x21,NULL,'80386');
INSERT INTO instruction VALUES('mov','Rq-Dq','x86_64',NULL,NULL,NULL,'secondary',0x21,NULL,'x86_64');
INSERT INTO instruction VALUES('mov','Rd-Td','legacy',NULL,NULL,NULL,'secondary',0x24,NULL,'80386');
INSERT INTO instruction VALUES('mov','Ob-:b',NULL,NULL,NULL,NULL,'primary',0xa2,NULL,'8086');
INSERT INTO instruction VALUES('mov','Ov-:v',NULL,NULL,NULL,NULL,'primary',0xa3,NULL,'8086');
INSERT INTO instruction VALUES('mov','Sw-Ev',NULL,NULL,NULL,NULL,'primary',0x8e,NULL,'8086');
INSERT INTO instruction VALUES('mov','Td-Rd','legacy',NULL,NULL,NULL,'secondary',0x26,NULL,'80386');

INSERT INTO instruction VALUES('movd','Ed-Pd',NULL,NULL,NULL,NULL,'secondary',0x7e,NULL,'mmx');
INSERT INTO instruction VALUES('movd','Pd-Ed',NULL,NULL,NULL,NULL,'secondary',0x6e,NULL,'mmx');
INSERT INTO instruction VALUES('movq','Eq-Pq','x86_64',NULL,'data_size_prefix_64bit',NULL,'secondary',0x7e,NULL,'mmx');
INSERT INTO instruction VALUES('movq','Pq-Eq','x86_64',NULL,'data_size_prefix_64bit',NULL,'secondary',0x6e,NULL,'mmx');
INSERT INTO instruction VALUES('movq','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x6f,NULL,'mmx');
INSERT INTO instruction VALUES('movq','Qq-Pq',NULL,NULL,NULL,NULL,'secondary',0x7f,NULL,'mmx');
INSERT INTO instruction VALUES('packssdw','Pq-Qd',NULL,NULL,NULL,NULL,'secondary',0x6b,NULL,'mmx');
INSERT INTO instruction VALUES('packsswb','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x63,NULL,'mmx');
INSERT INTO instruction VALUES('packuswb','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x67,NULL,'mmx');
INSERT INTO instruction VALUES('paddb','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xfc,NULL,'mmx');
INSERT INTO instruction VALUES('paddd','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xfe,NULL,'mmx');
INSERT INTO instruction VALUES('paddq','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xd4,NULL,'mmx');
INSERT INTO instruction VALUES('paddsb','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xec,NULL,'mmx');
INSERT INTO instruction VALUES('paddsw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xed,NULL,'mmx');
INSERT INTO instruction VALUES('paddusb','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xdc,NULL,'mmx');
INSERT INTO instruction VALUES('paddusw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xdd,NULL,'mmx');
INSERT INTO instruction VALUES('paddw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xfd,NULL,'mmx');
INSERT INTO instruction VALUES('pand','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xdb,NULL,'mmx');
INSERT INTO instruction VALUES('pandn','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xdf,NULL,'mmx');
INSERT INTO instruction VALUES('pcmpeqb','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x74,NULL,'mmx');
INSERT INTO instruction VALUES('pcmpeqd','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x76,NULL,'mmx');
INSERT INTO instruction VALUES('pcmpeqw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x75,NULL,'mmx');
INSERT INTO instruction VALUES('pcmpgtb','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x64,NULL,'mmx');
INSERT INTO instruction VALUES('pcmpgtd','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x66,NULL,'mmx');
INSERT INTO instruction VALUES('pcmpgtq','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x65,NULL,'mmx');
INSERT INTO instruction VALUES('pmaddwd','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xf5,NULL,'mmx');
INSERT INTO instruction VALUES('pmulhw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xe5,NULL,'mmx');
INSERT INTO instruction VALUES('pmullw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xd5,NULL,'mmx');
INSERT INTO instruction VALUES('por','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xeb,NULL,'mmx');
INSERT INTO instruction VALUES('pslld','Nq-Ib',NULL,NULL,NULL,NULL,'secondary',0x72,6,'mmx');
INSERT INTO instruction VALUES('pslld','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xf2,NULL,'mmx');
INSERT INTO instruction VALUES('psllq','Nq-Ib',NULL,NULL,NULL,NULL,'secondary',0x73,6,'mmx');
INSERT INTO instruction VALUES('psllq','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xf3,NULL,'mmx');
INSERT INTO instruction VALUES('psllw','Nq-Ib',NULL,NULL,NULL,NULL,'secondary',0x71,6,'mmx');
INSERT INTO instruction VALUES('psllw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xf1,NULL,'mmx');
INSERT INTO instruction VALUES('psrad','Nq-Ib',NULL,NULL,NULL,NULL,'secondary',0x72,4,'mmx');
INSERT INTO instruction VALUES('psrad','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xe2,NULL,'mmx');
INSERT INTO instruction VALUES('psraw','Nq-Ib',NULL,NULL,NULL,NULL,'secondary',0x71,4,'mmx');
INSERT INTO instruction VALUES('psraw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xe1,NULL,'mmx');
INSERT INTO instruction VALUES('psrld','Nq-Ib',NULL,NULL,NULL,NULL,'secondary',0x72,2,'mmx');
INSERT INTO instruction VALUES('psrld','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xd2,NULL,'mmx');
INSERT INTO instruction VALUES('psrlq','Nq-Ib',NULL,NULL,NULL,NULL,'secondary',0x73,2,'mmx');
INSERT INTO instruction VALUES('psrlq','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xd3,NULL,'mmx');
INSERT INTO instruction VALUES('psrlw','Nq-Ib',NULL,NULL,NULL,NULL,'secondary',0x71,2,'mmx');
INSERT INTO instruction VALUES('psrlw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xd1,NULL,'mmx');
INSERT INTO instruction VALUES('psubb','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xf8,NULL,'mmx');
INSERT INTO instruction VALUES('psubd','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xfa,NULL,'mmx');
INSERT INTO instruction VALUES('psubsw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xe9,NULL,'mmx');
INSERT INTO instruction VALUES('psubusb','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xd8,NULL,'mmx');
INSERT INTO instruction VALUES('psubusw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xd9,NULL,'mmx');
INSERT INTO instruction VALUES('psubw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xf9,NULL,'mmx');
INSERT INTO instruction VALUES('psussb','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xe8,NULL,'mmx');
INSERT INTO instruction VALUES('punpckhbw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x68,NULL,'mmx');
INSERT INTO instruction VALUES('punpckhdq','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x6a,NULL,'mmx');
INSERT INTO instruction VALUES('punpckhwd','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x69,NULL,'mmx');
INSERT INTO instruction VALUES('punpcklbw','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x60,NULL,'mmx');
INSERT INTO instruction VALUES('punpckldq','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x62,NULL,'mmx');
INSERT INTO instruction VALUES('punpcklwd','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0x61,NULL,'mmx');
INSERT INTO instruction VALUES('pxor','Pq-Qq',NULL,NULL,NULL,NULL,'secondary',0xef,NULL,'mmx');

INSERT INTO instruction VALUES('movs','Yb-Xb',NULL,NULL,NULL,NULL,'primary',0xa4,NULL,'8086');
INSERT INTO instruction VALUES('movs','Yv-Xv',NULL,NULL,NULL,NULL,'primary',0xa5,NULL,'8086');

INSERT INTO instruction VALUES('movsb',NULL,NULL,NULL,NULL,NULL,'primary',0xa4,NULL,'8086');
INSERT INTO instruction VALUES('movsd',NULL,NULL,NULL,'data_size_prefix_32bit',NULL,'primary',0xa5,NULL,'80386');
INSERT INTO instruction VALUES('movsq',NULL,'x86_64',NULL,'data_size_prefix_64bit',NULL,'primary',0xa5,NULL,'x86_64');
INSERT INTO instruction VALUES('movsw',NULL,NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0xa5,NULL,'8086');

INSERT INTO instruction VALUES('movsx','Gv-Eb',NULL,NULL,NULL,NULL,'secondary',0xbe,NULL,'80386');
INSERT INTO instruction VALUES('movsx','Gv-Ew',NULL,NULL,NULL,NULL,'secondary',0xbf,NULL,'80386');
INSERT INTO instruction VALUES('movsxd','Gd-Ed','x86_64',NULL,'data_size_prefix_32bit',NULL,'secondary',0x63,NULL,'x86_64');
INSERT INTO instruction VALUES('movsxd','Gq-Ed','x86_64',NULL,'data_size_prefix_64bit',NULL,'secondary',0x63,NULL,'x86_64');
INSERT INTO instruction VALUES('movsxd','Gw-Ew','x86_64',NULL,'data_size_prefix_16bit',NULL,'secondary',0x63,NULL,'x86_64');

INSERT INTO instruction VALUES('movzx','Gv-Eb',NULL,NULL,NULL,NULL,'secondary',0xb6,NULL,'80386');
INSERT INTO instruction VALUES('movzx','Gv-Ew',NULL,NULL,NULL,NULL,'secondary',0xb7,NULL,'80386');

INSERT INTO instruction VALUES('mul','Eb',NULL,NULL,NULL,NULL,'primary',0xf6,4,'8086');
INSERT INTO instruction VALUES('mul','Ev',NULL,NULL,NULL,NULL,'primary',0xf7,4,'8086');

INSERT INTO instruction VALUES('neg','Eb',NULL,NULL,NULL,NULL,'primary',0xf6,3,'8086');
INSERT INTO instruction VALUES('neg','Ev',NULL,NULL,NULL,NULL,'primary',0xf7,3,'8086');

INSERT INTO instruction VALUES('nop',NULL,NULL,NULL,NULL,NULL,'primary',0x90,NULL,'8086');
INSERT INTO instruction VALUES('nop','M',NULL,NULL,NULL,NULL,'secondary',0x1f,0,'nop');

INSERT INTO instruction VALUES('not','Eb',NULL,NULL,NULL,NULL,'primary',0xf6,2,'8086');
INSERT INTO instruction VALUES('not','Ev',NULL,NULL,NULL,NULL,'primary',0xf7,2,'8086');

INSERT INTO instruction VALUES('or',':b-Ib',NULL,NULL,NULL,NULL,'primary',0x0c,NULL,'8086');
INSERT INTO instruction VALUES('or',':v-Iz',NULL,NULL,NULL,NULL,'primary',0x0d,NULL,'8086');
INSERT INTO instruction VALUES('or','Eb-Gb',NULL,NULL,NULL,NULL,'primary',0x08,NULL,'8086');
INSERT INTO instruction VALUES('or','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0x80,1,'8086');
INSERT INTO instruction VALUES('or','Ev-Gv',NULL,NULL,NULL,NULL,'primary',0x09,NULL,'8086');
INSERT INTO instruction VALUES('or','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0x83,1,'8086');
INSERT INTO instruction VALUES('or','Ev-Iz',NULL,NULL,NULL,NULL,'primary',0x81,1,'8086');
INSERT INTO instruction VALUES('or','Gb-Eb',NULL,NULL,NULL,NULL,'primary',0x0a,NULL,'8086');
INSERT INTO instruction VALUES('or','Gv-Ev',NULL,NULL,NULL,NULL,'primary',0x0b,NULL,'8086');

INSERT INTO instruction VALUES('out','&w-:b',NULL,NULL,NULL,NULL,'primary',0xee,NULL,'8086');
INSERT INTO instruction VALUES('out','&w-:z',NULL,NULL,NULL,NULL,'primary',0xef,NULL,'8086');
INSERT INTO instruction VALUES('out','Ib-:b',NULL,NULL,NULL,NULL,'primary',0xe6,NULL,'8086');
INSERT INTO instruction VALUES('out','Ib-:z',NULL,NULL,NULL,NULL,'primary',0xe7,NULL,'8086');

INSERT INTO instruction VALUES('outs','&w-Xb',NULL,NULL,NULL,NULL,'primary',0x6e,NULL,'80186');
INSERT INTO instruction VALUES('outs','&w-Xz',NULL,NULL,NULL,NULL,'primary',0x6f,NULL,'80186');

INSERT INTO instruction VALUES('outsb',NULL,NULL,NULL,NULL,NULL,'primary',0x6c,NULL,'80186');
INSERT INTO instruction VALUES('outsw',NULL,NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0x6d,NULL,'80186');
INSERT INTO instruction VALUES('outsd',NULL,NULL,NULL,'data_size_prefix_32bit',NULL,'primary',0x6d,NULL,'80386');

INSERT INTO instruction VALUES('pause',NULL,NULL,NULL,NULL,'0xf3','primary',0x90,NULL,'8086');

INSERT INTO instruction VALUES('pop','=S',NULL,NULL,NULL,NULL,'secondary',0x81,NULL,'8086');
INSERT INTO instruction VALUES('pop','=q','x86_64',NULL,NULL,NULL,'primary',0x58,NULL,'8086');
INSERT INTO instruction VALUES('pop','=v','legacy',NULL,NULL,NULL,'primary',0x58,NULL,'8086');
INSERT INTO instruction VALUES('pop','=w','x86_64',NULL,'data_size_prefix_16bit',NULL,'primary',0x58,NULL,'8086');
INSERT INTO instruction VALUES('pop','Eq','x86_64',NULL,NULL,NULL,'primary',0x8f,0,'8086');
INSERT INTO instruction VALUES('pop','Ev','legacy',NULL,NULL,NULL,'primary',0x8f,0,'8086');
INSERT INTO instruction VALUES('pop','Ew','x86_64',NULL,'data_size_prefix_16bit',NULL,'primary',0x8f,0,'8086');
INSERT INTO instruction VALUES('popd','=S','legacy',NULL,'data_size_prefix_32bit',NULL,'secondary',0x81,NULL,'80386');
INSERT INTO instruction VALUES('popw','=S',NULL,NULL,'data_size_prefix_16bit',NULL,'secondary',0x81,NULL,'8086');
INSERT INTO instruction VALUES('popq','=S','x86_64',NULL,NULL,NULL,'secondary',0x81,NULL,'8086');

INSERT INTO instruction VALUES('popa',NULL,'legacy',NULL,NULL,NULL,'primary',0x61,NULL,'80186');
INSERT INTO instruction VALUES('popaw',NULL,'legacy',NULL,'data_size_prefix_16bit',NULL,'primary',0x61,NULL,'80186');
INSERT INTO instruction VALUES('popad',NULL,'legacy',NULL,'data_size_prefix_32bit',NULL,'primary',0x61,NULL,'80386');

INSERT INTO instruction VALUES('popf',NULL,'legacy',NULL,'data_size_prefix_16bit',NULL,'primary',0x9d,NULL,'8086');
INSERT INTO instruction VALUES('popfd',NULL,'legacy',NULL,'data_size_prefix_32bit',NULL,'primary',0x9d,NULL,'80386');
INSERT INTO instruction VALUES('popfq',NULL,'x86_64',NULL,NULL,NULL,'primary',0x9d,NULL,'x86_64');

INSERT INTO instruction VALUES('push','=S',NULL,NULL,NULL,NULL,'secondary',0x80,NULL,'8086');
INSERT INTO instruction VALUES('push','Eq','x86_64',NULL,NULL,NULL,'primary',0xff,6,'8086');
INSERT INTO instruction VALUES('push','Ev','legacy',NULL,NULL,NULL,'primary',0xff,6,'8086');
INSERT INTO instruction VALUES('push','Ew','x86_64',NULL,'data_size_prefix_16bit',NULL,'primary',0xff,6,'8086');
INSERT INTO instruction VALUES('push','=q','x86_64',NULL,NULL,NULL,'primary',0x50,NULL,'8086');
INSERT INTO instruction VALUES('push','=v','legacy',NULL,NULL,NULL,'primary',0x50,NULL,'8086');
INSERT INTO instruction VALUES('push','=w','x86_64',NULL,'data_size_prefix_16bit',NULL,'primary',0x50,NULL,'8086');
INSERT INTO instruction VALUES('push','Ib',NULL,NULL,NULL,NULL,'primary',0x6a,NULL,'80186');
INSERT INTO instruction VALUES('push','Iz',NULL,NULL,NULL,NULL,'primary',0x68,NULL,'80186');
INSERT INTO instruction VALUES('pushd','=S','legacy',NULL,'data_size_prefix_32bit',NULL,'secondary',0x80,NULL,'80386');
INSERT INTO instruction VALUES('pushw','=S',NULL,NULL,'data_size_prefix_16bit',NULL,'secondary',0x80,NULL,'8086');
INSERT INTO instruction VALUES('pushq','=S','x86_64',NULL,'data_size_prefix_64bit',NULL,'secondary',0x80,NULL,'x86_64');

INSERT INTO instruction VALUES('pusha',NULL,'legacy',NULL,NULL,NULL,'primary',0x60,NULL,'80186');
INSERT INTO instruction VALUES('pushaw',NULL,'legacy',NULL,'data_size_prefix_16bit',NULL,'primary',0x60,NULL,'80186');
INSERT INTO instruction VALUES('pushad',NULL,'legacy',NULL,'data_size_prefix_32bit',NULL,'primary',0x60,NULL,'80386');

INSERT INTO instruction VALUES('pushf',NULL,'legacy',NULL,'data_size_prefix_16bit',NULL,'primary',0x9d,NULL,'8086');
INSERT INTO instruction VALUES('pushfd',NULL,'legacy',NULL,'data_size_prefix_32bit',NULL,'primary',0x9d,NULL,'80386');
INSERT INTO instruction VALUES('pushfq',NULL,'x86_64',NULL,NULL,NULL,'primary',0x9d,NULL,'x86_64');

INSERT INTO instruction VALUES('rcl','Eb',NULL,NULL,NULL,NULL,'primary',0xd0,2,'8086');
INSERT INTO instruction VALUES('rcl','Ev',NULL,NULL,NULL,NULL,'primary',0xd1,2,'8086');
INSERT INTO instruction VALUES('rcl','Eb-#b',NULL,NULL,NULL,NULL,'primary',0xd2,2,'8086');
INSERT INTO instruction VALUES('rcl','Ev-#b',NULL,NULL,NULL,NULL,'primary',0xd3,2,'8086');
INSERT INTO instruction VALUES('rcl','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0xc0,2,'80186');
INSERT INTO instruction VALUES('rcl','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0xc1,2,'80186');

INSERT INTO instruction VALUES('rcr','Eb',NULL,NULL,NULL,NULL,'primary',0xd0,3,'8086');
INSERT INTO instruction VALUES('rcr','Ev',NULL,NULL,NULL,NULL,'primary',0xd1,3,'8086');
INSERT INTO instruction VALUES('rcr','Eb-#b',NULL,NULL,NULL,NULL,'primary',0xd2,3,'8086');
INSERT INTO instruction VALUES('rcr','Ev-#b',NULL,NULL,NULL,NULL,'primary',0xd3,3,'8086');
INSERT INTO instruction VALUES('rcr','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0xc0,3,'80186');
INSERT INTO instruction VALUES('rcr','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0xc1,3,'80186');

INSERT INTO instruction VALUES('rdmsr',NULL,NULL,NULL,NULL,NULL,'secondary',0x32,NULL,'msr');

INSERT INTO instruction VALUES('rdpmc',NULL,NULL,NULL,NULL,NULL,'secondary',0x33,NULL,'pmc');

INSERT INTO instruction VALUES('rdtsc',NULL,NULL,NULL,NULL,NULL,'secondary',0x31,NULL,'tsc');
INSERT INTO instruction VALUES('rdtscp',NULL,NULL,NULL,NULL,NULL,'secondary',0x01,0xf9,'rdtscp');

INSERT INTO instruction VALUES('retf',NULL,NULL,NULL,NULL,NULL,'primary',0xc3,NULL,'8086');
INSERT INTO instruction VALUES('retf','Iw',NULL,NULL,NULL,NULL,'primary',0xc2,NULL,'8086');

INSERT INTO instruction VALUES('retn',NULL,NULL,NULL,NULL,NULL,'primary',0xcb,NULL,'8086');
INSERT INTO instruction VALUES('retn','Iw',NULL,NULL,NULL,NULL,'primary',0xca,NULL,'8086');

INSERT INTO instruction VALUES('rol','Eb',NULL,NULL,NULL,NULL,'primary',0xd0,0,'8086');
INSERT INTO instruction VALUES('rol','Ev',NULL,NULL,NULL,NULL,'primary',0xd1,0,'8086');
INSERT INTO instruction VALUES('rol','Eb-#b',NULL,NULL,NULL,NULL,'primary',0xd2,0,'8086');
INSERT INTO instruction VALUES('rol','Ev-#b',NULL,NULL,NULL,NULL,'primary',0xd3,0,'8086');
INSERT INTO instruction VALUES('rol','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0xc0,0,'80186');
INSERT INTO instruction VALUES('rol','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0xc1,0,'80186');

INSERT INTO instruction VALUES('ror','Eb',NULL,NULL,NULL,NULL,'primary',0xd0,1,'8086');
INSERT INTO instruction VALUES('ror','Ev',NULL,NULL,NULL,NULL,'primary',0xd1,1,'8086');
INSERT INTO instruction VALUES('ror','Eb-#b',NULL,NULL,NULL,NULL,'primary',0xd2,1,'8086');
INSERT INTO instruction VALUES('ror','Ev-#b',NULL,NULL,NULL,NULL,'primary',0xd3,1,'8086');
INSERT INTO instruction VALUES('ror','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0xc0,1,'80186');
INSERT INTO instruction VALUES('ror','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0xc1,1,'80186');

INSERT INTO instruction VALUES('rsm',NULL,NULL,NULL,NULL,NULL,'secondary',0xaa,NULL,'80386');

INSERT INTO instruction VALUES('sahf',NULL,NULL,NULL,NULL,NULL,'primary',0x9e,NULL,'lahf');

INSERT INTO instruction VALUES('sal','Eb',NULL,NULL,NULL,NULL,'primary',0xd0,4,'8086');
INSERT INTO instruction VALUES('sal','Ev',NULL,NULL,NULL,NULL,'primary',0xd1,4,'8086');
INSERT INTO instruction VALUES('sal','Eb-#b',NULL,NULL,NULL,NULL,'primary',0xd2,4,'8086');
INSERT INTO instruction VALUES('sal','Ev-#b',NULL,NULL,NULL,NULL,'primary',0xd3,4,'8086');
INSERT INTO instruction VALUES('sal','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0xc0,4,'80186');
INSERT INTO instruction VALUES('sal','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0xc1,4,'80186');

INSERT INTO instruction VALUES('salc',NULL,'legacy',NULL,NULL,NULL,'primary',0xd6,NULL,'8086');

INSERT INTO instruction VALUES('sar','Eb',NULL,NULL,NULL,NULL,'primary',0xd0,7,'8086');
INSERT INTO instruction VALUES('sar','Ev',NULL,NULL,NULL,NULL,'primary',0xd1,7,'8086');
INSERT INTO instruction VALUES('sar','Eb-#b',NULL,NULL,NULL,NULL,'primary',0xd2,7,'8086');
INSERT INTO instruction VALUES('sar','Ev-#b',NULL,NULL,NULL,NULL,'primary',0xd3,7,'8086');
INSERT INTO instruction VALUES('sar','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0xc0,7,'80186');
INSERT INTO instruction VALUES('sar','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0xc1,7,'80186');

INSERT INTO instruction VALUES('sbb',':b-Ib',NULL,NULL,NULL,NULL,'primary',0x1c,NULL,'8086');
INSERT INTO instruction VALUES('sbb',':v-Iz',NULL,NULL,NULL,NULL,'primary',0x1d,NULL,'8086');
INSERT INTO instruction VALUES('sbb','Eb-Gb',NULL,NULL,NULL,NULL,'primary',0x18,NULL,'8086');
INSERT INTO instruction VALUES('sbb','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0x80,3,'8086');
INSERT INTO instruction VALUES('sbb','Ev-Gv',NULL,NULL,NULL,NULL,'primary',0x19,NULL,'8086');
INSERT INTO instruction VALUES('sbb','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0x83,3,'8086');
INSERT INTO instruction VALUES('sbb','Ev-Iz',NULL,NULL,NULL,NULL,'primary',0x81,3,'8086');
INSERT INTO instruction VALUES('sbb','Gb-Eb',NULL,NULL,NULL,NULL,'primary',0x1a,NULL,'8086');
INSERT INTO instruction VALUES('sbb','Gv-Ev',NULL,NULL,NULL,NULL,'primary',0x1b,NULL,'8086');

INSERT INTO instruction VALUES('scas',':b-Yb',NULL,NULL,NULL,NULL,'primary',0xae,NULL,'8086');
INSERT INTO instruction VALUES('scas','Yb',NULL,NULL,NULL,NULL,'primary',0xae,NULL,'8086');
INSERT INTO instruction VALUES('scas',':v-Yv',NULL,NULL,NULL,NULL,'primary',0xaf,NULL,'8086');
INSERT INTO instruction VALUES('scas','Yv',NULL,NULL,NULL,NULL,'primary',0xaf,NULL,'8086');

INSERT INTO instruction VALUES('scasb',NULL,NULL,NULL,NULL,NULL,'primary',0xae,NULL,'8086');
INSERT INTO instruction VALUES('scasw',NULL,NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0xaf,NULL,'8086');
INSERT INTO instruction VALUES('scasd',NULL,NULL,NULL,'data_size_prefix_32bit',NULL,'primary',0xaf,NULL,'80386');
INSERT INTO instruction VALUES('scasq',NULL,'x86_64',NULL,'data_size_prefix_64bit',NULL,'primary',0xaf,NULL,'x86_64');

INSERT INTO instruction VALUES('seta','Eb',NULL,NULL,NULL,NULL,'primary',0x97,0,'80386');
INSERT INTO instruction VALUES('setae','Eb',NULL,NULL,NULL,NULL,'primary',0x93,0,'80386');
INSERT INTO instruction VALUES('setb','Eb',NULL,NULL,NULL,NULL,'primary',0x92,0,'80386');
INSERT INTO instruction VALUES('setbe','Eb',NULL,NULL,NULL,NULL,'primary',0x96,0,'80386');
INSERT INTO instruction VALUES('sete','Eb',NULL,NULL,NULL,NULL,'primary',0x94,0,'80386');
INSERT INTO instruction VALUES('setg','Eb',NULL,NULL,NULL,NULL,'primary',0x9f,0,'80386');
INSERT INTO instruction VALUES('setge','Eb',NULL,NULL,NULL,NULL,'primary',0x9d,0,'80386');
INSERT INTO instruction VALUES('setl','Eb',NULL,NULL,NULL,NULL,'primary',0x9c,0,'80386');
INSERT INTO instruction VALUES('setle','Eb',NULL,NULL,NULL,NULL,'primary',0x9e,0,'80386');
INSERT INTO instruction VALUES('setne','Eb',NULL,NULL,NULL,NULL,'primary',0x95,0,'80386');
INSERT INTO instruction VALUES('setno','Eb',NULL,NULL,NULL,NULL,'primary',0x91,0,'80386');
INSERT INTO instruction VALUES('setnp','Eb',NULL,NULL,NULL,NULL,'primary',0x9b,0,'80386');
INSERT INTO instruction VALUES('setns','Eb',NULL,NULL,NULL,NULL,'primary',0x99,0,'80386');
INSERT INTO instruction VALUES('seto','Eb',NULL,NULL,NULL,NULL,'primary',0x90,0,'80386');
INSERT INTO instruction VALUES('setp','Eb',NULL,NULL,NULL,NULL,'primary',0x9a,0,'80386');
INSERT INTO instruction VALUES('sets','Eb',NULL,NULL,NULL,NULL,'primary',0x98,0,'80386');

INSERT INTO instruction VALUES('sgdt','Ms',NULL,NULL,NULL,NULL,'secondary',0x01,0,'80286');

INSERT INTO instruction VALUES('shl','Eb',NULL,NULL,NULL,NULL,'primary',0xd0,4,'8086');
INSERT INTO instruction VALUES('shl','Ev',NULL,NULL,NULL,NULL,'primary',0xd1,4,'8086');
INSERT INTO instruction VALUES('shl','Eb-#b',NULL,NULL,NULL,NULL,'primary',0xd2,4,'8086');
INSERT INTO instruction VALUES('shl','Ev-#b',NULL,NULL,NULL,NULL,'primary',0xd3,4,'8086');
INSERT INTO instruction VALUES('shl','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0xc0,4,'80186');
INSERT INTO instruction VALUES('shl','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0xc1,4,'80186');

INSERT INTO instruction VALUES('shld','Ev-Gv-Ib',NULL,NULL,NULL,NULL,'secondary',0xa4,NULL,'80386');
INSERT INTO instruction VALUES('shld','Ev-Gv-#b',NULL,NULL,NULL,NULL,'secondary',0xa5,NULL,'80386');

INSERT INTO instruction VALUES('shr','Eb',NULL,NULL,NULL,NULL,'primary',0xd0,5,'8086');
INSERT INTO instruction VALUES('shr','Ev',NULL,NULL,NULL,NULL,'primary',0xd1,5,'8086');
INSERT INTO instruction VALUES('shr','Eb-#b',NULL,NULL,NULL,NULL,'primary',0xd2,5,'8086');
INSERT INTO instruction VALUES('shr','Ev-#b',NULL,NULL,NULL,NULL,'primary',0xd3,5,'8086');
INSERT INTO instruction VALUES('shr','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0xc0,5,'80186');
INSERT INTO instruction VALUES('shr','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0xc1,5,'80186');

INSERT INTO instruction VALUES('shrd','Ev-Gv-Ib',NULL,NULL,NULL,NULL,'secondary',0xac,NULL,'80386');
INSERT INTO instruction VALUES('shrd','Ev-Gv-#b',NULL,NULL,NULL,NULL,'secondary',0xad,NULL,'80386');

INSERT INTO instruction VALUES('sidt','Ms',NULL,NULL,NULL,NULL,'secondary',0x01,1,'80286');

INSERT INTO instruction VALUES('sldt','Ew',NULL,NULL,NULL,NULL,'secondary',0x00,0,'80286');

INSERT INTO instruction VALUES('smsw','Mw',NULL,NULL,NULL,NULL,'secondary',0x01,4,'80286');
INSERT INTO instruction VALUES('smsw','Rd',NULL,NULL,'data_size_prefix_32bit',NULL,'secondary',0x01,4,'80286');
INSERT INTO instruction VALUES('smsw','Rq','x86_64',NULL,'data_size_prefix_64bit',NULL,'secondary',0x01,4,'80286');
INSERT INTO instruction VALUES('smsw','Rw',NULL,NULL,'data_size_prefix_16bit',NULL,'secondary',0x01,4,'80286');

INSERT INTO instruction VALUES('stc',NULL,NULL,NULL,NULL,NULL,'primary',0xf9,NULL,'8086');

INSERT INTO instruction VALUES('std',NULL,NULL,NULL,NULL,NULL,'primary',0xfd,NULL,'8086');

INSERT INTO instruction VALUES('sti',NULL,NULL,NULL,NULL,NULL,'primary',0xfb,NULL,'8086');

INSERT INTO instruction VALUES('str','Ew',NULL,NULL,NULL,NULL,'secondary',0x00,1,'80286');

INSERT INTO instruction VALUES('stos','Yb',NULL,NULL,NULL,NULL,'primary',0xaa,NULL,'8086');
INSERT INTO instruction VALUES('stos','Yb-:b',NULL,NULL,NULL,NULL,'primary',0xaa,NULL,'8086');
INSERT INTO instruction VALUES('stos','Yv',NULL,NULL,NULL,NULL,'primary',0xab,NULL,'8086');
INSERT INTO instruction VALUES('stos','Yv-:v',NULL,NULL,NULL,NULL,'primary',0xab,NULL,'8086');

INSERT INTO instruction VALUES('stosb',NULL,NULL,NULL,NULL,NULL,'primary',0xaa,NULL,'8086');
INSERT INTO instruction VALUES('stosw',NULL,NULL,NULL,'data_size_prefix_16bit',NULL,'primary',0xab,NULL,'8086');
INSERT INTO instruction VALUES('stosd',NULL,NULL,NULL,'data_size_prefix_32bit',NULL,'primary',0xab,NULL,'80386');
INSERT INTO instruction VALUES('stosq',NULL,'x86_64',NULL,'data_size_prefix_64bit',NULL,'primary',0xab,NULL,'x86_64');

INSERT INTO instruction VALUES('sub',':b-Ib',NULL,NULL,NULL,NULL,'primary',0x2c,NULL,'8086');
INSERT INTO instruction VALUES('sub',':v-Iz',NULL,NULL,NULL,NULL,'primary',0x2d,NULL,'8086');
INSERT INTO instruction VALUES('sub','Eb-Gb',NULL,NULL,NULL,NULL,'primary',0x28,NULL,'8086');
INSERT INTO instruction VALUES('sub','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0x80,5,'8086');
INSERT INTO instruction VALUES('sub','Ev-Gv',NULL,NULL,NULL,NULL,'primary',0x29,NULL,'8086');
INSERT INTO instruction VALUES('sub','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0x83,5,'8086');
INSERT INTO instruction VALUES('sub','Ev-Iz',NULL,NULL,NULL,NULL,'primary',0x81,5,'8086');
INSERT INTO instruction VALUES('sub','Gb-Eb',NULL,NULL,NULL,NULL,'primary',0x2a,NULL,'8086');
INSERT INTO instruction VALUES('sub','Gv-Ev',NULL,NULL,NULL,NULL,'primary',0x2b,NULL,'8086');

INSERT INTO instruction VALUES('swapgs',NULL,NULL,NULL,NULL,NULL,'secondary',0x01,0xf8,'x86_64');

INSERT INTO instruction VALUES('syscall',NULL,NULL,NULL,NULL,NULL,'secondary',0x05,NULL,'syscall');

INSERT INTO instruction VALUES('sysenter',NULL,NULL,NULL,NULL,NULL,'secondary',0x34,NULL,'sep');

INSERT INTO instruction VALUES('sysexit',NULL,NULL,NULL,NULL,NULL,'secondary',0x35,NULL,'sep');

INSERT INTO instruction VALUES('sysret',NULL,NULL,NULL,NULL,NULL,'secondary',0x07,NULL,'syscall');

INSERT INTO instruction VALUES('test',':b-Ib',NULL,NULL,NULL,NULL,'primary',0xa8,NULL,'8086');
INSERT INTO instruction VALUES('test',':v-Iz',NULL,NULL,NULL,NULL,'primary',0xa9,NULL,'8086');
INSERT INTO instruction VALUES('test','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0xf6,0,'8086');
INSERT INTO instruction VALUES('test','Eb-Gb',NULL,NULL,NULL,NULL,'primary',0x84,NULL,'8086');
INSERT INTO instruction VALUES('test','Ev-Iz',NULL,NULL,NULL,NULL,'primary',0xf7,0,'8086');
INSERT INTO instruction VALUES('test','Ev-Gv',NULL,NULL,NULL,NULL,'primary',0x85,NULL,'8086');

INSERT INTO instruction VALUES('tzcnt','Gv-Ev',NULL,NULL,NULL,'0xf3','secondary',0xbc,NULL,'bmi1');

INSERT INTO instruction VALUES('ud0',NULL,NULL,NULL,NULL,NULL,'secondary',0xff,NULL,'80186');
INSERT INTO instruction VALUES('ud1','Gv-M',NULL,NULL,NULL,NULL,'secondary',0xb9,NULL,'80186');
INSERT INTO instruction VALUES('ud1','Gv-Rv',NULL,NULL,NULL,NULL,'secondary',0xb9,NULL,'80186');
INSERT INTO instruction VALUES('ud2',NULL,NULL,NULL,NULL,NULL,'secondary',0x0b,NULL,'80186');

INSERT INTO instruction VALUES('verr','Ew',NULL,NULL,NULL,NULL,'secondary',0x00,4,'80286');

INSERT INTO instruction VALUES('verw','Ew',NULL,NULL,NULL,NULL,'secondary',0x00,5,'80286');

INSERT INTO instruction VALUES('wait',NULL,NULL,NULL,NULL,NULL,'primary',0x9b,NULL,'8086');

INSERT INTO instruction VALUES('wbinvd',NULL,NULL,NULL,NULL,NULL,'secondary',0x09,NULL,'80486');

INSERT INTO instruction VALUES('wrmsr',NULL,NULL,NULL,NULL,NULL,'secondary',0x30,NULL,'msr');

INSERT INTO instruction VALUES('xadd','Eb-Gb',NULL,NULL,NULL,NULL,'secondary',0xc0,NULL,'80486');
INSERT INTO instruction VALUES('xadd','Ev-Gv',NULL,NULL,NULL,NULL,'secondary',0xc1,NULL,'80486');

INSERT INTO instruction VALUES('xbts','Gv-Ev',NULL,NULL,NULL,NULL,'secondary',0xa6,NULL,'80386');
INSERT INTO instruction VALUES('xbts','Gv-Ev-:v-#b',NULL,NULL,NULL,NULL,'secondary',0xa6,NULL,'80386');

INSERT INTO instruction VALUES('xchg',':v-=v',NULL,NULL,NULL,NULL,'primary',0x90,NULL,'8086');
INSERT INTO instruction VALUES('xchg','=v-:v',NULL,NULL,NULL,NULL,'primary',0x90,NULL,'8086');
INSERT INTO instruction VALUES('xchg','Eb-Gb',NULL,NULL,NULL,NULL,'primary',0x86,NULL,'8086');
INSERT INTO instruction VALUES('xchg','Ev-Gv',NULL,NULL,NULL,NULL,'primary',0x87,NULL,'8086');
INSERT INTO instruction VALUES('xchg','Gb-Eb',NULL,NULL,NULL,NULL,'primary',0x86,NULL,'8086');
INSERT INTO instruction VALUES('xchg','Gv-Ev',NULL,NULL,NULL,NULL,'primary',0x87,NULL,'8086');

INSERT INTO instruction VALUES('xlat','%b',NULL,NULL,NULL,NULL,'primary',0xd7,NULL,'8086');
INSERT INTO instruction VALUES('xlat',':b-%b',NULL,NULL,NULL,NULL,'primary',0xd7,NULL,'8086');
INSERT INTO instruction VALUES('xlatb',NULL,NULL,NULL,NULL,NULL,'primary',0xd7,NULL,'8086');

INSERT INTO instruction VALUES('xor',':b-Ib',NULL,NULL,NULL,NULL,'primary',0x34,NULL,'8086');
INSERT INTO instruction VALUES('xor',':v-Iz',NULL,NULL,NULL,NULL,'primary',0x35,NULL,'8086');
INSERT INTO instruction VALUES('xor','Eb-Gb',NULL,NULL,NULL,NULL,'primary',0x30,NULL,'8086');
INSERT INTO instruction VALUES('xor','Eb-Ib',NULL,NULL,NULL,NULL,'primary',0x80,6,'8086');
INSERT INTO instruction VALUES('xor','Ev-Gv',NULL,NULL,NULL,NULL,'primary',0x31,NULL,'8086');
INSERT INTO instruction VALUES('xor','Ev-Ib',NULL,NULL,NULL,NULL,'primary',0x83,6,'8086');
INSERT INTO instruction VALUES('xor','Ev-Iz',NULL,NULL,NULL,NULL,'primary',0x81,6,'8086');
INSERT INTO instruction VALUES('xor','Gb-Eb',NULL,NULL,NULL,NULL,'primary',0x32,NULL,'8086');
INSERT INTO instruction VALUES('xor','Gv-Ev',NULL,NULL,NULL,NULL,'primary',0x33,NULL,'8086');
COMMIT;
