use std::borrow::BorrowMut;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::ptr::write;
use std::str::{Chars, FromStr};
use std::string::ParseError;
use std::thread::current;

use itertools::{fold, Itertools};
use phf::phf_map;
use tailcall::tailcall;

const E1: &str = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581";
const INPUT: &str = "9165985876154699219988192797299938946759548993842382179897895847959995468789384779887969965834298757\n9998264799399739852669159899969915752881369928979589425659869512849898348591988899941938989958981368\n8899439372928798295981284899995498957198997822776999766989269673341115866632499916582179985999797178\n9421373314798816327241583824298987799745811978869899899899999689988933652499891999899718759652641398\n9949959989598519929632977671926983547787167773939739198988988851889519228348573848979828599971727943\n2989192448779756984992587956689999842746119939998567894718899918799954281648927282191699499764931166\n9536499426969676189784443598979559899499773119978188394689557995599331996979736693529419681251956996\n1751936936954979779895998917863998697858138691879749999727789764741997539275579593798975539798818972\n7274746616894977772987689459988988891794769772199789185982726479992789563994126143759648659653627929\n9768211178649269995923493912176968666679999859829913848396992289798892197419967914777851877899477278\n9548265381888989898295865115497574817515178999881998168798969539989897979118423366393898169329939816\n5346816194398971492996938739782725986596757996472498951666679779287752997484725936582558299359964812\n2169889968977588483272986596967998729999883958728879796999737989942928997399898577324799379798165679\n5756791285489939716695977945746649394898965799971791856215827997699758829299499895496993375817688889\n1189911823962199612255959754716953775367595418858921381973577497988994381699599376948136788285998274\n6979939299876267486998569159464199464965974658469723835541132825855912999935783995325776519991867869\n7829881839997878488734339898145124968389716637926975397882799631985728918519668146942356911683694975\n4973997156985297758563699888285794535932928974966978296562139199889893476728999942416259889912711471\n9534797994999788999759474799734844192166842694971197691854895641895188586895455192589986798997913693\n2819784988295365719599997589858664998979399989735399884134499895873389479651295996498866989999839974\n6129944129969453989699888866976269999128469799982997963676678293639494188967181417362922546475281598\n9716591619494494558731969871917781639763288288691962634219628479989988828962651246459129568974491171\n7894482759988943587818779691955666934841799931371572868778856815865991124376396974593984947897442991\n1858394839149329949299818139987736884848178995897668499996188998667699618978779495959989927996996397\n9917119977481935978699197129381282957978683881779735689583195993458959989897975199999285945815997999\n1699128644295779552889817864638995225998895287579281281998594696644213479297199692979781937997179875\n8289472999575769888986161678994171392789714988719932799199558696689379278929798887977551878788398787\n9175199787918919864185866129967892999898318993291829953849829788999738939999469848949591989149962712\n6199185639948988878676462732119197225882925435874993975768952939825978994996892549999699984912695356\n3396488583898545189449831859997919671368765298895692847986994925119921865818912189499168979958896494\n9938398336984195997576689194134985999635325783571526493894394589917988793989755746999698829649589393\n6985999497989855798918845568728129349777627978979362788359497952698956998292996918648692699694349887\n1983998898992942689898989791697792913121476987631657959299998868769559979688918876893688862439185197\n7357776772998794811999929459919571597899775797198394831478974157594549666253368399586349899987699999\n7959339994696971922632949191545959757987989979787788169979748972545188912811999284691666989915589771\n9759961834557989443271499976572889278289319995879484972649589489199519573998999779383692242897767588\n9179641759649515678926897874488858988897787895855979191899776967749898123692484838554413919817918963\n6839761379993114699928685489349481791394595895995488717149328139993956495742445794747189959976877649\n7719888678899926948979191678165949542927483596385943836398298659699148778343914768999898197267997991\n8769727958885176392169934146934172488871783986753744485932511888999799159194751538987186924419884479\n8991999992999929999897984599767977994699827768813861298818993288386943869782797583698682188885283399\n7618891897968939798498999399278379698962758659288163913969875574557443483997692961986999939899819481\n8879168556577788339549879912587944289995777798881688244889799389891799876819699576957557971877898128\n9291695492736749179768777989972456198819896897386591181956469899994155718299785968686938589318979479\n7795999789358882518386787487224292238877893956695912892699678996798249199324897986249725799479399977\n7157587585956489829119899759896997988918879818491999931184192141949199172799356679472486896195864474\n8369998998796539973399979987949986863639829688617494995142799799778639869918868177475771699198897648\n6597979998999646797717999135983617949772192975367951599248888799957499785994774771558359664883693989\n8494399497884369731998936499887638658943929616778716995355827188745777858983426799559721799739362629\n7988688889359569698815957687229757873596999199938929146584679599188685439814466838789683219554419476\n9247298485768968989794782295879689497761879956994316114992795924987262939579993889586584967699911179\n9585318399982677997578979671589977319169586337196999959189521499887711992343851295981988819951696391\n9688419987598317988779256198579659993398274893319912676925791429994831998147985462785322692958417991\n9162526572947889939619891135679859992512868978725888837198497899147199825914711544192984934699878858\n9997992172487298739215471619688889939819838188921969391991648949979166298795194179866892977317488668\n4342969513889787361777751972798659462495119377122327857968869995779944989446988786669168689799978198\n9991989792887933999782849992885388199539897175961949179928388186994694998865752811971681796835228899\n9689884899572994167989994913659819996941825788877898819911788969419991277989227389959829597962878797\n9919879157714445923163829999735888889724892834556637899899999974386932299127993655996239789769999544\n8158429263197997788393974998999939783878693949897825898975995959979986635699272784592895953298769979\n6933156399191389719992767671896649494989689977121886269912939799779176467784638649979932992778611947\n9937886748328953482685979758934658999819989845699372899934596991259755741819979779889881397565568579\n6628287411551868899648999511774894539834357749799788481594933187788979179772699757813897655969637759\n1681758889993977881987844998586378799289969999542735618971489988849699628668119898999679199493949189\n7756952966791755589777886975949697297729368719914568929921989979278297994486928996985666791996513546\n7959939142776881868999785876799728991368773296952962918971941199687791336941297836969992566198117899\n9919799591383641992451999795686291882995991159937526287799259396499782592953878378919512331399769295\n9979988697799299993248298599458499952548969526718987894849849961566959196296836979299696595499951269\n8989115299788834687596939899183482448889849883899881337161943668283395379982966119363885868799817686\n3917979748934696925996487991958936699899582264499928897937594999993391587881813378359971812919779972\n7798825938413749319999785941489868587674397449269738978317389938919984622199581498529938819682972471\n7749948181969657924978787838596695869966969369158962427829991499668977295896254897998989743989952389\n6398998955793159938819887788968371888968395499298999755899949268632898528951445886293997299392969779\n9539983773686993498991392899889572558299175987819981797981886747694169489893727157979378595839496298\n9599999287193787789729986389617989992374479972959749982818939999895692869845931449861767839389829149\n4889176836968695418797941267779399488818276299865898498649489796971967884999112828966259271689857259\n7139139187955994831738585298962698949981181887589699969919418116293969946431635243579918986885919619\n7935837589876972417397869798312741789988882795986987744696696629647813993997762979599189148298958581\n8791284382878199766798982957837371997961791491996313339915819373928948979938596612197699691473969914\n1372974376943738718696489825854989786382983999879965536283589995655483569334269859559761199879679291\n9719671746885959293824693968698399315424948599622489865269469961617669916732853892695261177787994496\n7849319899791243818799879975258532467699699999398269787199341236894749619399991856788917729181194197\n9148263323757681139949799197198549259446781912979561927929891898923997238737858757915394778591127667\n1945591989843987198391756487965892217958111951642219915715439797998675786222936815998966826782589594\n5254778838276178988551266998775337643868756772978159294191819424888551874271745598136847865594999289\n3998557499888479397893111869792537426834985941959185929798919875628859952429849886849639853349182496\n8958877677623879221669563361998988956937492798913659491897715599879989983979859984661929198589817564\n9989969281947994946512996153988898382127889834868897498898867797998892148557494839482788798269898496\n9779851919689459958899912662756981619298821434879339749956713988385759469959788279765789991925947918\n4196471827218751217999955385399979199189581896987488518319752529829892282899638773984698854832217793\n5968399569814182278918739959691249997958587277596698779311494824187981838837892479347738267879912541\n8659894118598996577145167494795592898597175979829589886499183913197276893979278479678988218798894991\n6649277899864442637895792699719172999998662937819979979769821897696686788719291967114346689819212978\n6699889979874997773889958977455779952988466968759738452847569993821963796198741514377779188577811889\n1173976886399735895884999486498511998399588328832288555979166889776686858695414825689877994898917949\n9629147139882191719857926675389957799296279989442991999622669639755767892922474778888299984452999898\n4391879947595258989144156986799899833458894496218777872997391217987623967998848296898891312888421579\n9362699299759899879589911968947655251988997896874791778876389919939496782959848979971163281597161779\n1652179878596567755189593919723287659886989991199789326995581692999815373488857495418291969165215199\n9788389477191759669998798979888832998683445278988415997594187969167388689899997659918914818788982937";

fn part1() {
    let mut input: Vec<Vec<u32>> = INPUT.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u32).collect::<Vec<u32>>())
        .collect();

    // Note: This now takes really long! On my laptop, it took me ~70min with the --release flag
    // enabled. Part 2 output was: 3016
    input = enlarge_input(input);

    // print input

    for line in &input {
        for x in line {
            print!("{}", x);
        }
        println!();
    }

    let width = input[0].len();
    let height = input.len();

    let mut distances = create_empty_distances(width, height);
    let mut precursors = create_empty_precursors(width, height);
    let mut q = create_initial_q(width, height);

    // println!("{:?}", distances);
    // println!("{:?}", precursors);
    // println!("{:?}", q);

    // dijkstra

    while !q.is_empty() {
        let u = vertex_from_q_where_distance_is_minimal(&q, &distances);
        q.remove(&u);
        if u == (width - 1, height - 1) { break; }

        for neighbor in neighbors(u.0, u.1, width, height) {
            if q.contains(&neighbor) {
                let alt = distances.get(&u).unwrap().unwrap() + &input[neighbor.1][neighbor.0];
                let neighbors_distance = distances.get(&neighbor).unwrap();
                if neighbors_distance.is_none() || alt < neighbors_distance.unwrap() {
                    distances.insert(neighbor, Some(alt));
                    precursors.insert(neighbor, Some(u));
                }


                //  1  Methode distanz_update(u,v,abstand[],vorgänger[]):
                //  2      alternativ:= abstand[u] + abstand_zwischen(u, v)   // Weglänge vom Startknoten nach v über u
                //  3      falls alternativ < abstand[v]:
                //  4          abstand[v]:= alternativ
                //  5          vorgänger[v]:= u
            }
        }
    }


    //  3      solange Q nicht leer:                       // Der eigentliche Algorithmus
    //  4          u:= Knoten in Q mit kleinstem Wert in abstand[]
    //  5          entferne u aus Q                                // für u ist der kürzeste Weg nun bestimmt
    //  6          für jeden Nachbarn v von u:
    //  7              falls v in Q:                            // falls noch nicht berechnet
    //  8                 distanz_update(u,v,abstand[],vorgänger[])   // prüfe Abstand vom Startknoten zu v
    //  9      return vorgänger[]

    println!("{:?}", precursors);
    println!("{:?}", precursors.get(&(8, 8)));

    // calculate total risk

    let mut current_point = (width - 1, height - 1);
    let mut total_risk = 0;

    while current_point != (0, 0) {
        total_risk += input[current_point.1][current_point.0];
        current_point = precursors.get(&current_point).unwrap().unwrap();
    }

    println!("{}", total_risk);
}

fn enlarge_input(input: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = Vec::new();

    // enlarge from left to right

    for line in input {
        let mut new_line = Vec::new();

        // 11637517422274862853338597396444961841755517295286
        // 116375174222748628533385973964449608407555071951866618206297

        for i in 0..5 {
            for x in &line {
                let value = if x + i <= 9 { x + i } else { (x + i) % 9 };
                new_line.push(value);
            }
        }

        result.push(new_line);
    }

    // enlarge from top to bottom

    let current_len = result.len();

    for i in 1..5 {
        for y in 0..current_len {
            let original_line = result.get(y).unwrap().clone();
            result.push(original_line.iter()
                .map(|x| if x + i <= 9 { x + i } else { (x + i) % 9 })
                .collect());
        }
    }

    result
}

fn vertex_from_q_where_distance_is_minimal(q: &HashSet<(usize, usize)>, distances: &HashMap<(usize, usize), Option<u32>>) -> (usize, usize) {
    let mut min_distance = None;
    let mut min_pos = None;

    for qi in q {
        let x = distances.get(qi).unwrap();
        if let &Some(distance) = x {
            if min_distance == None || distance < min_distance.unwrap() {
                min_distance = Some(distance);
                min_pos = Some(*qi);
            }
        }
    }

    min_pos.unwrap()
}

#[derive(Eq, PartialEq, Debug, Ord)]
struct Node {
    point: (usize, usize),
    cost: Option<u32>,
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

fn create_initial_q(width: usize, height: usize) -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            set.insert((x, y));
        }
    }
    set

    // let mut heap = BinaryHeap::new();
    // for x in 0..width {
    //     for y in 0..height {
    //         if (x, y) == (0, 0) {
    //             heap.push(Reverse(Node { point: (x, y), cost: Some(0) }))
    //         } else {
    //             heap.push(Reverse(Node { point: (x, y), cost: None }));
    //         }
    //     }
    // }
}

fn create_empty_precursors(width: usize, height: usize) -> HashMap<(usize, usize), Option<(usize, usize)>> {
    let mut map = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            map.insert((x, y), None);
        }
    }
    map
}

fn create_empty_distances(width: usize, height: usize) -> HashMap<(usize, usize), Option<u32>> {
    let mut map = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            map.insert((x, y), None);
        }
    }
    map.insert((0, 0), Some(0));
    map
}

fn neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    if y > 0 { neighbors.push((x, y - 1)); }
    if y < height - 1 { neighbors.push((x, y + 1)); }
    if x > 0 { neighbors.push((x - 1, y)); }
    if x < width - 1 { neighbors.push((x + 1, y)) }

    neighbors
}

fn part2() {}

fn main() {
    part1();
}