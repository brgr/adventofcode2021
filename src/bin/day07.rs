use itertools::Itertools;

const E1: &str = "16,1,2,0,4,2,7,1,2,14";
const INPUT: &str = "1101,1,29,67,1102,0,1,65,1008,65,35,66,1005,66,28,1,67,65,20,4,0,1001,65,1,65,1106,0,8,99,35,67,101,99,105,32,110,39,101,115,116,32,112,97,115,32,117,110,101,32,105,110,116,99,111,100,101,32,112,114,111,103,114,97,109,10,78,78,1006,298,320,1021,308,639,76,65,531,302,936,125,502,481,793,369,213,25,158,1423,1387,996,519,138,16,1348,1390,957,24,67,126,35,576,333,788,1707,845,278,732,512,1187,1203,153,1126,15,297,200,43,454,901,176,994,1268,1833,195,1,1705,373,175,417,410,511,63,244,136,552,433,8,347,546,1167,899,127,852,192,14,337,1174,982,228,255,56,881,444,520,144,1461,822,545,22,298,1479,950,569,48,105,396,42,314,173,47,134,769,99,204,33,470,811,797,327,104,1309,29,122,22,562,1804,1318,1116,736,71,1093,39,1381,852,75,937,1635,674,750,42,142,334,788,206,224,395,61,112,26,96,111,421,56,253,417,452,899,401,345,13,745,10,1075,42,867,76,1013,614,1011,1108,180,300,128,62,190,99,478,89,501,815,471,1014,39,39,66,550,421,308,485,484,339,552,74,65,502,975,90,165,360,91,302,136,46,238,308,478,264,720,394,24,321,360,371,754,434,53,632,167,352,254,249,201,719,251,181,142,1028,593,502,246,810,171,28,133,520,0,1080,68,1031,1058,972,89,913,237,880,114,914,447,838,118,361,1185,590,376,36,31,663,201,1088,323,164,322,842,1551,834,266,51,246,862,763,1319,560,1213,1301,919,400,1220,337,526,790,22,41,377,313,346,40,1480,176,548,1160,29,465,1542,171,18,367,1250,227,298,643,602,314,258,36,525,1011,1058,351,27,154,56,361,164,199,311,15,314,74,412,932,1107,290,597,589,66,1815,97,155,252,1022,902,6,284,577,16,566,1167,911,42,591,99,1360,400,1231,955,292,545,1453,780,946,204,526,22,43,1471,608,299,36,473,119,1169,247,617,366,345,835,1133,744,209,1388,78,970,221,81,1034,4,654,990,79,629,1480,651,271,785,12,188,1103,55,714,227,420,63,828,494,321,200,354,1686,691,718,595,1290,431,375,240,40,1436,92,230,60,3,409,1111,622,1567,216,372,558,2,969,33,730,39,782,931,686,129,1081,507,83,696,168,109,26,568,304,76,698,584,1041,1123,152,521,289,140,11,18,1247,1729,747,48,642,244,445,657,1564,1302,325,253,183,42,149,152,63,61,893,176,451,654,482,407,589,780,1911,83,267,491,1370,534,613,350,1296,1097,258,1182,15,150,1064,777,722,16,62,689,834,297,33,195,677,36,884,1026,855,376,70,466,821,704,946,1309,779,1421,870,335,139,358,108,34,71,965,249,445,1583,232,183,68,1802,667,1361,428,136,1094,1233,1506,174,1198,610,68,855,1067,253,1209,283,5,245,1,277,1745,632,104,866,161,476,608,70,218,184,475,115,871,678,153,45,133,6,29,523,31,1523,996,1442,915,378,413,21,336,53,11,420,237,1039,356,365,236,222,213,122,1056,1313,553,81,470,976,41,287,247,30,500,201,45,147,173,102,116,206,868,691,335,160,608,1401,526,1187,965,547,690,16,55,6,231,272,1180,3,142,419,213,24,983,20,15,321,445,913,95,248,6,260,1298,316,449,401,384,831,60,4,115,1357,289,10,1022,446,4,53,853,1097,115,1470,220,63,485,408,151,1376,741,1036,460,636,663,322,701,529,770,240,1226,1075,53,6,1334,568,963,893,402,270,803,667,1501,1406,699,256,810,417,3,182,359,703,54,1122,1113,355,597,445,656,129,43,259,1915,1091,613,473,593,317,1139,257,39,1041,621,535,121,320,1661,455,483,2,27,525,687,32,366,1577,192,859,191,1102,410,327,113,479,76,1772,589,470,768,1151,896,74,181,342,12,797,146,447,583,913,24,61,183,658,454,17,27,958,429,450,1518,762,10,1044,38,936,1554,421,470,474,504,96,650,769,1180,597,50,193,52,806,263,106,734,791,134,150,69,273,118,896,185,12,1955,544,908,962,818,965,135,1105,677,0,321,642,355,180,290,1498,1793,331,214,10,405,169,374,184,309,311,1405,39,1378,631,974,204,771,1195,145,1083,403,717,250,1524,1683,868,69,293,1228,163,1093,737,895,443,11,450,1282,668,335,546,1470,332,169,325,407,80,173,1441,553,478,903,239,330,414,3,1145,383,194,373,1333,474,1147,1189,436,70,987,130,1245,195,780,15,0,48,24,718,1305,897,935,279,1187,640,269,3,465,754,821,9,669,90,391,1081,302,190,591,57,607,544,1068,1152,95,436,73,486,302,758,129,435,1777,72,291,267,85,731,113,431,930,3,100,602,133,480,822,289,189,54,230,205,8,123,247,566,947";


fn part1() {
    let values: Vec<u64> = INPUT.split(",").map(|n| n.parse().unwrap()).collect();
    println!("{:?}", values);

    let mut min_fuel: i64 = -1;

    for i in 0..10_000 {
        let mut fuel: u64 = 0;

        for x in &values {
            let n = if i > *x { i - x } else { x - i };
            fuel += (n * (n + 1)) / 2;
        }

        if min_fuel == -1 || (fuel as i64) < min_fuel {
            min_fuel = fuel as i64;
        }
    }

    println!("{}", min_fuel);

}

fn main() {
    part1();
}