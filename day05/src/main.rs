use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::str::FromStr;
use std::string::ParseError;

use inputs;

const EXAMPLE: &str = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2\n";
const INPUT: &str = "62,963 -> 844,181\n58,85 -> 917,944\n137,76 -> 137,347\n453,125 -> 347,19\n178,65 -> 977,864\n447,360 -> 62,745\n723,326 -> 156,893\n47,497 -> 107,437\n387,491 -> 340,491\n58,477 -> 283,252\n86,351 -> 562,827\n215,172 -> 539,172\n496,801 -> 496,63\n546,412 -> 232,98\n621,807 -> 481,807\n471,20 -> 618,20\n175,283 -> 175,467\n19,283 -> 19,290\n159,137 -> 159,11\n593,181 -> 543,181\n167,976 -> 929,976\n730,782 -> 959,782\n713,285 -> 713,880\n583,144 -> 583,296\n39,61 -> 961,983\n778,81 -> 604,81\n70,560 -> 70,889\n85,129 -> 666,710\n689,688 -> 632,688\n76,52 -> 903,879\n510,543 -> 22,55\n510,935 -> 470,935\n780,357 -> 780,602\n440,349 -> 710,79\n934,801 -> 412,801\n979,25 -> 35,969\n379,527 -> 379,76\n243,524 -> 243,664\n534,945 -> 11,422\n198,367 -> 224,367\n871,451 -> 456,451\n226,231 -> 939,231\n686,354 -> 740,300\n543,68 -> 340,68\n506,160 -> 319,347\n177,25 -> 177,603\n337,450 -> 724,450\n421,519 -> 676,519\n858,976 -> 179,297\n236,222 -> 236,250\n254,242 -> 254,626\n859,243 -> 23,243\n89,982 -> 979,92\n58,758 -> 101,801\n930,483 -> 587,826\n667,717 -> 667,762\n512,816 -> 845,816\n17,501 -> 17,760\n345,61 -> 847,61\n531,840 -> 618,840\n67,748 -> 262,748\n548,461 -> 163,846\n934,142 -> 169,907\n119,931 -> 580,470\n769,916 -> 457,604\n587,458 -> 93,458\n109,850 -> 768,191\n225,129 -> 160,64\n544,163 -> 544,476\n304,594 -> 61,351\n510,396 -> 510,741\n772,210 -> 772,889\n867,415 -> 721,269\n466,266 -> 466,44\n305,609 -> 305,237\n563,962 -> 451,962\n566,402 -> 28,940\n889,717 -> 891,717\n754,545 -> 313,545\n930,976 -> 209,255\n70,911 -> 692,289\n737,37 -> 958,37\n652,566 -> 720,634\n776,551 -> 370,957\n484,476 -> 820,476\n119,420 -> 639,420\n394,964 -> 394,221\n340,767 -> 964,143\n715,289 -> 481,55\n236,389 -> 826,389\n747,642 -> 33,642\n583,351 -> 244,690\n609,17 -> 609,680\n460,365 -> 668,365\n519,180 -> 929,590\n206,45 -> 782,45\n507,185 -> 386,306\n16,12 -> 982,978\n31,348 -> 320,348\n54,975 -> 947,82\n844,714 -> 870,714\n677,965 -> 677,699\n387,699 -> 387,26\n329,479 -> 189,479\n970,708 -> 538,708\n565,434 -> 565,623\n748,737 -> 748,497\n255,984 -> 255,600\n146,59 -> 932,845\n191,929 -> 423,929\n316,409 -> 802,409\n208,560 -> 559,209\n885,237 -> 135,987\n477,486 -> 260,486\n845,59 -> 845,811\n225,369 -> 162,369\n858,678 -> 858,362\n162,972 -> 27,972\n828,26 -> 283,571\n670,48 -> 114,604\n732,487 -> 620,487\n570,575 -> 14,19\n113,203 -> 162,154\n374,702 -> 374,452\n850,575 -> 535,575\n841,133 -> 841,474\n976,960 -> 642,960\n177,428 -> 177,246\n969,289 -> 589,289\n787,842 -> 731,786\n743,709 -> 336,709\n15,914 -> 299,630\n863,952 -> 17,952\n586,889 -> 586,512\n442,128 -> 436,128\n633,367 -> 79,921\n21,990 -> 257,990\n829,297 -> 829,103\n975,633 -> 879,633\n946,887 -> 72,13\n531,720 -> 123,312\n84,954 -> 815,223\n989,982 -> 257,982\n669,417 -> 928,158\n128,935 -> 87,976\n692,850 -> 191,850\n686,856 -> 686,259\n135,396 -> 473,58\n837,206 -> 629,206\n751,227 -> 751,900\n190,617 -> 190,502\n850,265 -> 254,265\n229,587 -> 325,491\n980,747 -> 465,232\n54,375 -> 439,375\n737,844 -> 711,844\n533,219 -> 123,629\n232,805 -> 232,798\n911,441 -> 911,160\n80,294 -> 80,527\n880,533 -> 590,533\n674,84 -> 674,670\n956,440 -> 554,842\n24,939 -> 890,73\n516,183 -> 145,554\n71,584 -> 71,766\n629,173 -> 643,187\n34,360 -> 639,965\n983,871 -> 983,682\n986,590 -> 986,327\n769,986 -> 130,986\n392,192 -> 70,192\n577,379 -> 635,379\n243,664 -> 162,664\n273,987 -> 273,192\n251,548 -> 558,855\n989,736 -> 989,611\n400,697 -> 134,431\n646,923 -> 646,841\n768,782 -> 386,782\n93,973 -> 939,127\n489,91 -> 489,551\n313,683 -> 248,748\n986,61 -> 201,846\n322,413 -> 737,413\n567,716 -> 567,614\n198,624 -> 439,624\n402,198 -> 147,453\n897,352 -> 897,298\n773,379 -> 773,19\n373,256 -> 931,814\n690,796 -> 543,796\n884,368 -> 464,368\n136,864 -> 622,378\n458,569 -> 458,254\n491,462 -> 491,412\n558,340 -> 73,340\n980,52 -> 980,605\n126,609 -> 390,345\n437,659 -> 17,659\n53,928 -> 982,928\n389,591 -> 389,832\n464,46 -> 464,754\n646,680 -> 646,988\n919,159 -> 109,969\n334,75 -> 219,75\n976,639 -> 976,685\n264,773 -> 128,773\n787,771 -> 699,771\n415,124 -> 549,124\n468,71 -> 468,701\n815,121 -> 797,121\n619,95 -> 610,104\n886,294 -> 120,294\n148,136 -> 148,314\n816,971 -> 454,971\n888,733 -> 431,733\n59,836 -> 840,55\n52,965 -> 962,55\n989,982 -> 19,12\n697,818 -> 185,306\n883,638 -> 481,638\n429,285 -> 170,26\n516,507 -> 516,301\n767,102 -> 61,808\n764,793 -> 209,238\n568,411 -> 261,718\n706,622 -> 685,622\n226,110 -> 790,674\n544,429 -> 544,334\n794,588 -> 794,792\n804,738 -> 782,738\n370,552 -> 370,189\n960,275 -> 644,275\n133,896 -> 686,896\n12,986 -> 987,11\n978,973 -> 69,64\n92,465 -> 62,465\n733,57 -> 18,57\n110,845 -> 110,272\n123,935 -> 123,499\n37,960 -> 986,11\n332,209 -> 344,221\n237,279 -> 349,279\n875,635 -> 875,420\n552,174 -> 552,635\n10,93 -> 853,936\n909,82 -> 909,926\n511,743 -> 511,830\n223,974 -> 223,124\n829,543 -> 11,543\n307,671 -> 206,570\n126,72 -> 956,72\n528,903 -> 528,223\n644,524 -> 952,216\n734,324 -> 734,105\n225,558 -> 225,159\n667,122 -> 667,64\n582,93 -> 582,509\n817,932 -> 727,932\n898,18 -> 79,837\n12,987 -> 986,13\n426,79 -> 722,79\n496,884 -> 906,884\n953,183 -> 953,508\n360,881 -> 975,881\n765,862 -> 579,862\n14,55 -> 14,560\n454,333 -> 290,333\n19,479 -> 91,551\n696,41 -> 56,41\n329,203 -> 812,203\n498,559 -> 498,636\n822,852 -> 614,852\n410,370 -> 410,624\n829,415 -> 805,415\n775,980 -> 204,980\n705,780 -> 116,191\n49,30 -> 988,969\n324,199 -> 554,199\n727,572 -> 157,572\n212,693 -> 93,693\n886,105 -> 152,105\n239,834 -> 958,115\n623,920 -> 623,523\n389,225 -> 106,508\n443,426 -> 443,108\n129,770 -> 858,41\n906,559 -> 392,559\n44,793 -> 774,793\n693,275 -> 693,738\n623,434 -> 184,873\n774,623 -> 774,895\n140,187 -> 140,238\n247,503 -> 45,301\n575,365 -> 950,365\n101,120 -> 646,120\n42,682 -> 649,75\n749,767 -> 516,534\n551,53 -> 73,531\n15,26 -> 885,896\n749,15 -> 235,529\n548,169 -> 784,405\n458,564 -> 962,564\n663,873 -> 678,873\n349,773 -> 349,927\n777,180 -> 637,320\n238,306 -> 844,912\n927,818 -> 652,543\n404,673 -> 952,125\n750,297 -> 18,297\n926,958 -> 926,669\n767,843 -> 767,833\n151,136 -> 234,219\n927,789 -> 468,330\n593,361 -> 593,447\n48,14 -> 954,920\n282,972 -> 790,972\n537,446 -> 202,446\n847,125 -> 357,615\n667,609 -> 299,609\n820,987 -> 359,987\n342,889 -> 595,889\n692,414 -> 239,414\n916,935 -> 70,89\n289,884 -> 289,790\n264,562 -> 373,562\n850,24 -> 126,748\n877,159 -> 213,823\n702,607 -> 702,454\n432,883 -> 432,260\n530,387 -> 229,387\n783,39 -> 783,933\n757,775 -> 757,81\n416,376 -> 474,376\n220,462 -> 220,824\n438,317 -> 421,317\n403,312 -> 866,312\n902,923 -> 204,923\n345,33 -> 819,33\n376,521 -> 549,521\n172,320 -> 129,277\n25,975 -> 976,24\n730,108 -> 465,373\n607,468 -> 737,598\n376,55 -> 672,55\n807,113 -> 974,113\n345,804 -> 695,454\n687,921 -> 650,884\n262,743 -> 262,753\n889,734 -> 499,344\n424,727 -> 909,242\n100,957 -> 100,832\n558,958 -> 376,958\n422,473 -> 539,356\n424,463 -> 158,463\n329,543 -> 816,543\n300,74 -> 362,136\n620,691 -> 620,312\n215,727 -> 360,582\n692,116 -> 618,116\n945,722 -> 945,560\n851,83 -> 450,484\n692,424 -> 254,862\n160,214 -> 160,405\n937,101 -> 854,184\n989,14 -> 18,985\n256,275 -> 828,847\n797,748 -> 509,748\n521,148 -> 422,148\n85,549 -> 85,807\n689,688 -> 443,442\n750,664 -> 648,562\n51,616 -> 51,54\n925,272 -> 925,696\n284,560 -> 369,560\n509,685 -> 509,559\n985,157 -> 273,869\n570,765 -> 614,721\n62,981 -> 985,58\n289,496 -> 289,104\n752,232 -> 692,292\n82,948 -> 683,948\n15,20 -> 984,989\n252,950 -> 252,132\n930,659 -> 614,659\n552,449 -> 798,695\n850,894 -> 342,386\n412,465 -> 412,383\n249,616 -> 351,718\n759,289 -> 613,289\n673,347 -> 673,842\n749,493 -> 449,493\n378,468 -> 378,674\n914,924 -> 890,900\n514,56 -> 606,56\n855,233 -> 979,233\n170,756 -> 170,961\n450,601 -> 450,87\n868,192 -> 125,935\n702,137 -> 231,608\n109,36 -> 632,36\n511,472 -> 511,945\n208,884 -> 923,169\n831,66 -> 146,66\n435,133 -> 884,133\n900,418 -> 916,418\n957,104 -> 127,104\n608,892 -> 608,40\n554,782 -> 55,782\n305,260 -> 305,712\n942,143 -> 226,859\n823,778 -> 317,778\n228,415 -> 228,445\n313,505 -> 669,505\n43,539 -> 43,187\n14,84 -> 743,813\n687,101 -> 277,101\n549,977 -> 549,392\n21,637 -> 214,637\n950,961 -> 104,115\n778,831 -> 958,831\n214,765 -> 579,765\n586,42 -> 89,42\n505,950 -> 505,115\n144,734 -> 144,813\n11,349 -> 11,681\n49,336 -> 99,386\n560,187 -> 560,551\n678,602 -> 761,519\n131,515 -> 411,795\n957,835 -> 957,106\n948,852 -> 948,990\n541,946 -> 541,405\n355,147 -> 724,516\n644,476 -> 625,476\n789,818 -> 207,236\n259,57 -> 431,57\n441,375 -> 441,34\n774,121 -> 882,13\n655,397 -> 188,864\n467,432 -> 235,200\n268,121 -> 268,842\n975,14 -> 11,978\n124,904 -> 935,93\n401,582 -> 420,582\n170,700 -> 523,347\n20,681 -> 20,174\n420,939 -> 173,692\n61,933 -> 956,38\n686,458 -> 686,939\n780,561 -> 305,86\n792,644 -> 792,780\n632,550 -> 938,550\n441,252 -> 841,252\n789,59 -> 789,418\n981,11 -> 278,714\n264,41 -> 264,186\n870,833 -> 605,568\n160,905 -> 160,783\n385,191 -> 385,403\n774,791 -> 69,86\n409,967 -> 409,173\n868,41 -> 868,235\n536,497 -> 949,497\n757,119 -> 156,720\n563,706 -> 883,706\n124,482 -> 14,482\n353,655 -> 904,104\n194,868 -> 194,649\n810,736 -> 748,736\n815,578 -> 50,578\n531,131 -> 241,131\n18,972 -> 977,13\n761,747 -> 73,59\n650,701 -> 930,701\n470,237 -> 470,740\n333,803 -> 954,182\n644,667 -> 235,667\n943,766 -> 299,766\n985,876 -> 985,503\n170,924 -> 467,924\n249,19 -> 981,751\n462,666 -> 462,651\n404,228 -> 877,228\n174,440 -> 174,847\n910,596 -> 672,596\n430,663 -> 734,663\n711,294 -> 69,294\n193,302 -> 257,302\n959,20 -> 13,966\n171,561 -> 171,953\n704,986 -> 29,311\n285,886 -> 285,260\n945,872 -> 531,458\n265,748 -> 478,748\n26,537 -> 26,851\n205,210 -> 917,922\n590,488 -> 241,139\n536,179 -> 247,179";


#[derive(Debug)]
struct Line {
    from: (u32, u32),
    to: (u32, u32),
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" -> ").collect();

        let from: Vec<&str> = split[0].split(",").collect();
        let to: Vec<&str> = split[1].split(",").collect();

        Ok(Line {
            from: (from[0].parse::<u32>().unwrap(), from[1].parse::<u32>().unwrap()),
            to: (to[0].parse::<u32>().unwrap(), to[1].parse::<u32>().unwrap()),
        })
    }
}

impl Line {
    pub fn x_range(&self) -> Vec<u32> {
        if self.from.0 <= self.to.0 {
            return (self.from.0..=self.to.0).collect();
        } else {
            return (self.to.0..=self.from.0).rev().collect();
        }
    }

    pub fn y_range(&self) -> Vec<u32> {
        if self.from.1 <= self.to.1 {
            return (self.from.1..=self.to.1).collect();
        } else {
            return (self.to.1..=self.from.1).rev().collect();
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.from.1 == self.to.1
    }

    pub fn is_vertical(&self) -> bool {
        self.from.0 == self.to.0
    }

    /** Note: Currently only works with horizontal / vertical */
    pub fn get_points(&self) -> Vec<(u32, u32)> {
        if self.is_horizontal() {
            return self.x_range().iter()
                .map(|x| (*x, self.from.1))
                .collect();
        } else if self.is_vertical() {
            return self.y_range().iter()
                .map(|y| (self.from.0, *y))
                .collect();
        }

        panic!("Only implemented for vertical and horizontal!")
    }
}

type Board = HashMap<(u32, u32), u32>;

trait CountBoard {
    fn add_to_board(&mut self, point: (u32, u32));
    fn get_count(&self, point: (u32, u32)) -> &u32;
}

impl CountBoard for Board {
    fn add_to_board(&mut self, point: (u32, u32)) {
        let new_value = self.get(&point).unwrap_or(&0u32) + 1;
        self.insert(point, new_value);
    }

    fn get_count(&self, point: (u32, u32)) -> &u32 {
        self.get(&point).unwrap_or(&0u32)
    }
}

fn part1(input: Vec<Line>) {
    let considered_lines: Vec<&Line> = input.iter()
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .collect();

    println!("{:?}", considered_lines);

    let mut board = Board::new();
    considered_lines.iter()
        .for_each(|line| line.get_points().into_iter()
            .for_each(|point| board.add_to_board(point)));

    // let max = 1000000;
    // let all_points: Vec<(u32, u32)> = (0u32..=max)
    //     .map(|x|
    //         (0u32..=max).map(|y| (x, y))
    //             .collect::<Vec<(u32, u32)>>())
    //     .flatten()
    //     .collect();

    // println!("{:?}", board.iter().collect());

    let intersected_at = board.values()
        // .map(|point| board.get_count(point))
        .filter(|count| count >= &&2)
        .count();

    println!("{}", intersected_at);
}

fn part2(inputs: Vec<String>) {}

fn main() {
    // let inputs: Vec<String> = inputs::get_input_split(3);
    let example: Vec<Line> = INPUT.lines().map(|s| Line::from_str(s).unwrap()).collect();

    for x in 2..=0 {
        println!("{}", x);
    }

    println!("{:?}", example);

    part1(example);
}
