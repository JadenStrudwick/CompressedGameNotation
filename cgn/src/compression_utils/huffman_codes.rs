use huffman_compress::{Book, CodeBuilder, Tree};
use std::collections::HashMap;

pub fn convert_hashmap_to_weights(hashmap: &HashMap<u8, u32>) -> (Book<u8>, Tree<u8>) {
    CodeBuilder::from_iter(hashmap).finish()
}

pub fn get_lichess_hashmap() -> HashMap<u8, u32> {
    let mut weights: HashMap<u8, u32> = HashMap::new();
    weights.insert(0, 225_883_932);
    weights.insert(1, 134_956_126);
    weights.insert(2, 89_041_269);
    weights.insert(3, 69_386_238);
    weights.insert(4, 57_040_790);
    weights.insert(5, 44_974_559);
    weights.insert(6, 36_547_155);
    weights.insert(7, 31_624_920);
    weights.insert(8, 28_432_772);
    weights.insert(9, 26_540_493);
    weights.insert(10, 24_484_873);
    weights.insert(11, 23_058_034);
    weights.insert(12, 23_535_272);
    weights.insert(13, 20_482_457);
    weights.insert(14, 20_450_172);
    weights.insert(15, 18_316_057);
    weights.insert(16, 17_214_833);
    weights.insert(17, 16_964_761);
    weights.insert(18, 16_530_028);
    weights.insert(19, 15_369_510);
    weights.insert(20, 14_178_440);
    weights.insert(21, 14_275_714);
    weights.insert(22, 13_353_306);
    weights.insert(23, 12_829_602);
    weights.insert(24, 13_102_592);
    weights.insert(25, 11_932_647);
    weights.insert(26, 10_608_657);
    weights.insert(27, 10_142_459);
    weights.insert(28, 8_294_594);
    weights.insert(29, 7_337_490);
    weights.insert(30, 6_337_744);
    weights.insert(31, 5_380_717);
    weights.insert(32, 4_560_556);
    weights.insert(33, 3_913_313);
    weights.insert(34, 3_038_767);
    weights.insert(35, 2_480_514);
    weights.insert(36, 1_951_026);
    weights.insert(37, 1_521_451);
    weights.insert(38, 1_183_252);
    weights.insert(39, 938_708);
    weights.insert(40, 673_339);
    weights.insert(41, 513_153);
    weights.insert(42, 377_299);
    weights.insert(43, 276_996);
    weights.insert(44, 199_682);
    weights.insert(45, 144_602);
    weights.insert(46, 103_313);
    weights.insert(47, 73046);
    weights.insert(48, 52339);
    weights.insert(49, 36779);
    weights.insert(50, 26341);
    weights.insert(51, 18719);
    weights.insert(52, 13225);
    weights.insert(53, 9392);
    weights.insert(54, 6945);
    weights.insert(55, 4893);
    weights.insert(56, 3698);
    weights.insert(57, 2763);
    weights.insert(58, 2114);
    weights.insert(59, 1631);
    weights.insert(60, 1380);
    weights.insert(61, 1090);
    weights.insert(62, 887);
    weights.insert(63, 715);
    weights.insert(64, 590);
    weights.insert(65, 549);
    weights.insert(66, 477);
    weights.insert(67, 388);
    weights.insert(68, 351);
    weights.insert(69, 319);
    weights.insert(70, 262);
    weights.insert(71, 236);
    weights.insert(72, 200);
    weights.insert(73, 210);
    weights.insert(74, 153);
    weights.insert(75, 117);
    weights.insert(76, 121);
    weights.insert(77, 121);
    weights.insert(78, 115);
    weights.insert(79, 95);
    weights.insert(80, 75);
    weights.insert(81, 67);
    weights.insert(82, 55);
    weights.insert(83, 50);
    weights.insert(84, 55);
    weights.insert(85, 33);
    weights.insert(86, 33);
    weights.insert(87, 30);
    weights.insert(88, 32);
    weights.insert(89, 28);
    weights.insert(90, 29);
    weights.insert(91, 27);
    weights.insert(92, 21);
    weights.insert(93, 15);
    weights.insert(94, 9);
    weights.insert(95, 10);
    weights.insert(96, 12);
    weights.insert(97, 12);
    weights.insert(98, 8);
    weights.insert(99, 7);
    weights.insert(100, 2);
    weights.insert(101, 4);
    weights.insert(102, 5);
    weights.insert(103, 5);
    weights.insert(104, 0);
    weights.insert(105, 5);
    weights.insert(106, 1);
    weights.insert(107, 1);
    weights.insert(108, 0);
    weights.insert(109, 1);
    weights.insert(110, 2);
    weights.insert(111, 1);
    weights.insert(112, 1);
    weights.insert(113, 0);
    weights.insert(114, 0);
    weights.insert(115, 1);
    weights.insert(116, 0);
    weights.insert(117, 0);
    weights.insert(118, 0);
    weights.insert(119, 0);
    weights.insert(120, 0);
    weights.insert(121, 0);
    weights.insert(122, 0);
    weights.insert(123, 0);
    weights.insert(124, 0);
    weights.insert(125, 0);
    weights.insert(126, 0);
    weights.insert(127, 0);
    weights.insert(128, 0);
    weights.insert(129, 0);
    weights.insert(130, 0);
    weights.insert(131, 0);
    weights.insert(132, 0);
    weights.insert(133, 0);
    weights.insert(134, 0);
    weights.insert(135, 0);
    weights.insert(136, 0);
    weights.insert(137, 0);
    weights.insert(138, 0);
    weights.insert(139, 0);
    weights.insert(140, 0);
    weights.insert(141, 0);
    weights.insert(142, 0);
    weights.insert(143, 0);
    weights.insert(144, 0);
    weights.insert(145, 0);
    weights.insert(146, 0);
    weights.insert(147, 0);
    weights.insert(148, 0);
    weights.insert(149, 0);
    weights.insert(150, 0);
    weights.insert(151, 0);
    weights.insert(152, 0);
    weights.insert(153, 0);
    weights.insert(154, 0);
    weights.insert(155, 0);
    weights.insert(156, 0);
    weights.insert(157, 0);
    weights.insert(158, 0);
    weights.insert(159, 0);
    weights.insert(160, 0);
    weights.insert(161, 0);
    weights.insert(162, 0);
    weights.insert(163, 0);
    weights.insert(164, 0);
    weights.insert(165, 0);
    weights.insert(166, 0);
    weights.insert(167, 0);
    weights.insert(168, 0);
    weights.insert(169, 0);
    weights.insert(170, 0);
    weights.insert(171, 0);
    weights.insert(172, 0);
    weights.insert(173, 0);
    weights.insert(174, 0);
    weights.insert(175, 0);
    weights.insert(176, 0);
    weights.insert(177, 0);
    weights.insert(178, 0);
    weights.insert(179, 0);
    weights.insert(180, 0);
    weights.insert(181, 0);
    weights.insert(182, 0);
    weights.insert(183, 0);
    weights.insert(184, 0);
    weights.insert(185, 0);
    weights.insert(186, 0);
    weights.insert(187, 0);
    weights.insert(188, 0);
    weights.insert(189, 0);
    weights.insert(190, 0);
    weights.insert(191, 0);
    weights.insert(192, 0);
    weights.insert(193, 0);
    weights.insert(194, 0);
    weights.insert(195, 0);
    weights.insert(196, 0);
    weights.insert(197, 0);
    weights.insert(198, 0);
    weights.insert(199, 0);
    weights.insert(200, 0);
    weights.insert(201, 0);
    weights.insert(202, 0);
    weights.insert(203, 0);
    weights.insert(204, 0);
    weights.insert(205, 0);
    weights.insert(206, 0);
    weights.insert(207, 0);
    weights.insert(208, 0);
    weights.insert(209, 0);
    weights.insert(210, 0);
    weights.insert(211, 0);
    weights.insert(212, 0);
    weights.insert(213, 0);
    weights.insert(214, 0);
    weights.insert(215, 0);
    weights.insert(216, 0);
    weights.insert(217, 0);
    weights.insert(218, 0);
    weights.insert(219, 0);
    weights.insert(220, 0);
    weights.insert(221, 0);
    weights.insert(222, 0);
    weights.insert(223, 0);
    weights.insert(224, 0);
    weights.insert(225, 0);
    weights.insert(226, 0);
    weights.insert(227, 0);
    weights.insert(228, 0);
    weights.insert(229, 0);
    weights.insert(230, 0);
    weights.insert(231, 0);
    weights.insert(232, 0);
    weights.insert(233, 0);
    weights.insert(234, 0);
    weights.insert(235, 0);
    weights.insert(236, 0);
    weights.insert(237, 0);
    weights.insert(238, 0);
    weights.insert(239, 0);
    weights.insert(240, 0);
    weights.insert(241, 0);
    weights.insert(242, 0);
    weights.insert(243, 0);
    weights.insert(244, 0);
    weights.insert(245, 0);
    weights.insert(246, 0);
    weights.insert(247, 0);
    weights.insert(248, 0);
    weights.insert(249, 0);
    weights.insert(250, 0);
    weights.insert(251, 0);
    weights.insert(252, 0);
    weights.insert(253, 0);
    weights.insert(254, 0);
    weights.insert(255, 0);
    weights
}
