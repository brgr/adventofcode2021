use phf::phf_map;
use std::collections::{HashMap, HashSet};
use std::str::{Chars, FromStr};
use std::string::ParseError;

use itertools::Itertools;



const E1: &str = "[({(<(())[]>[[{[]{<()<>>";
const E2: &str = "{([(<{}[<>[]}>{[]{[(<()>";
const E3: &str = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]";
const INPUT: &str = "[[{[<{{<<{<[<{<>[]}([][])>{{<>()}<()>}](({{}<>}[[]<>])<([]<>){[]()}>)>[<{{<>}<[]()>}[{[][]}<{}{}>]><{<\n{[<[({([[(<<(([]{}))<[{}()]>>>)]])<([<{[{({}{})[[]()]}[(()[])<{}()]]]{{[[][]]<<>[]>}}}{([([\n{<{<{(({[[([{<<><>>([]())}[{()[]}({}())]}<{{{}}{[]<>}}({<>{}})>){<(([]())([]<>))(<<>[]><[]<>>)>({[[]\n{[([<{<([{{(({()<>}({}<>)))(<{<>{}}](<<>[]><()()>))}{[{{[]()}(<>[])}{{[][]}{<>{}}}]}}{[{{{{}[]}<<>()>}}<{([]\n[({[{[(<<<[((<{}<>><{}{}>)({[]<>}((){}))){{[{}<>]{{}<>}}})[<[<<>>[<>{}]]{{<>[]}}>]>>[[([<{<><>}\n<[{[{[[<{[[{<<[]()>[{}<>]>}]<([{{}}({}{})])>]{<[[<<><>><()[]>]([[]<>]{[]<>})]{(([][]){()[]})\n((<(<(<[(<([(<{}<>>(()()))](<{{}[]}{()[]}>))<<<(<>[]){()()}>([<>{}]{{}})>({{[][]}(()())}(<\n{{([(<{<([([[[<><>][()<>]]{(()[])}])]({<[<<>>[()<>]]({()[]}[{}()])>]))>({<<[<{[]()}<<>[]>>{<[]\n([<{{((<({<[<({}<>)[[]()]>]([(<>)<(){}>](([][]){{}{}}))>{(({<>[]}<()()>){(()[])})[{{[][]}((){})}[([]<>)({}{})\n<<<{<{<([<[((<()<>]<[]()>)[(<>[])[{}<>]])]><[<[(()())[()()]]{[{}{}][()()]}>]{<<(<>())[<>[]]>>[<(()())<[]\n{<[[({<[{[{[({{}<>}<<>{}>){<{}[]><{}{}>}]{{<<>>}<(<>()){()<>}>}}]}{{<[({{}()})([()<>]{[]})\n(((([[(((<(<<(<>())[(){}]>([[][]]{()()})>{<[<>[]][()<>]>(([]{})[<>{}])})[({{()<>}({}())})]>([[[(()<>)][[<>()\n[<({([[[<<([<<<>()>>{[<>()](<>{})}]){<[[{}<>]]<{{}<>}([]{})>>}>[<<<{{}()}>>[[([][])<<><>>]({<>\n{<(<<{{<<[({{({}<>)[[][]]}(<()[]>[<>])}<{<()>{{}()}}>){{<[(){}][(){}>><{{}<>}[[]{}]>}}]<{<{\n{(<{<[<(<<(<[([][])<[]{}>]><{([]())(<>{})}{<(){}><[][]>}>><({({}[]){<>()}}{<{}>(())})>>{({{(<><>)[{}()]}\n[(<{[<<[{((({[{}<>][[]<>]}){(<{}<>>({}<>))<(<><>)[<>]>})<({{<>[]}}}>)}]>([{{<<{{[][]}(<>())}<{{}\n<<<[[{[<<{[{<(()[])<[]()>>}[(({}[]))(({}<>){<>})]](([{(){}}[<>]]<[{}{}]<{}<>>>))}>{{[(((()[]){(){}})(<(){}>{\n[{{<<([[[[<<[{(){}}{(){}}]{<{}()>(()())}>{{<{}()>}<[<><>][()()]>)>([{{()<>}<()[]>}[[<><>]{[]<>}]]{[{\n<[[{[{<[(({({[<>{}]}([<>[]]))<<[<><>]([]<>)>([{}()][()()))>}[((<{}>{[]{}}))[[{{}}[<><>]]]])(([{{[]()}<[]\n{{{<{([[[<[<{{(){}}({}{})}>[[{()[]}<[]{}>]<<()<>>[()[]]>]}<{[[[]()]<<><>>]({<><>}<[][]>)}{(<[]()><{}<>>)}>>\n([<<[(([{{[([{<>{}}])<[(()){<>{}}]>](<{([][])[<>()]}<{{}()}>>)}>{[{{(<()()>[<><>])((<><>){()()})}{([[][]])[[{\n[{((({<([{(<{(<>)[[]()]}({{}<>}<[]<>>)>)}(<<[({}{})]{({}<>)[<>()]}>>[<[[()<>]{<>()}][<[][]><<>{}>]>[<([][])[(\n(<[{<[<((<{<[{<>[]}{{}<>}]([[][]]{{}[]})>[<[(){}]{[]()}>({<>}{<><>}}]}>[[(<{()[]}<()[]>>[{{}()}<<>[]>\n({{<([{<<<<<{{()}({}())}<[()]{[]{}}>>[([()<>]([][]))<{()<>}>]>>>>}<{[[[<<<<>()>{[]{}}>[({}()){(){}}]\n<[{([<<(<(<[(({}()){[][]})[[(){}][[]()]]][<[{}]([][])>]>}>(<{[({[]<>}{(){}})<(<>)({}())>]<[({}<>)<{\n(({[{<({<<[<{{[]<>}[<><>]}{{{}}<()[]>}>]<<<{()()}{{}<>}>([[]()][<>()])>>><[([(<><>)<()[]>]\n{([<<[{[(({[{{()<>}({}{})}<[{}[]]<(){}>>]<{<[]<>>}<[()()][()()]>>}<<[[(){}](<><>)]><(<{}[]>\n{([<{<([(<(<(<[]{}>{<>()}){{<>{}}}>({<[][]>(<>{})}))<[<[{}{}]<{}{}>>{(<>[])(<>())}][<{{}<>}(()[]\n[{[(<(({{(<{(<[][]>){[<>{}]}}{<{(){}}{<><>}>({[]{}}{[]{}})}>)(<[(([]<>))[[[]()]<[]<>>]]<{({}[])<{}<>>}[\n[{<{({((<{[{[<<>{}>[[]]]{<{}()>[<>]}>]}>[{([(<[][]>{[]()}){<(){}>[<>[]]}][[([]<>)<[]{}>]{{{}}[{}]}])([\n{<(<({[{[(({((<>[])<{}{}>)({[]<>>[{}[]])}([{(){}}([])]{<<>[]>[{}()]}))[[<[<><>]<<><>>>{[()()]<\n{{{<[<<{(<(([({}<>)<()>]{<()()><[]<>>})(<[()[]]{[]{}}>{{{}{}}(<>())}))({([[]()]{()[]}){{{}<>}[\n(<({{[{<{[<[[{[]()}]([[]()][<>{}])][[[<>]{<>}]<([]{})<{}[]>>]>{[{{<><>}[[][]]}<{[]{}}({}())>](<[[]()]({\n<<[{[<{<[[(<[<(){}>(<>)](<[]<>>[[]()])>{<<{}[]><{}()>>[(()[])(<>{})]})({({{}<>}[<>{}])}{({[][]}<<><>>)})]({<[\n[[([<{{(<({[{([][])(()<>)}]{{{[]<>}[()()]}[({}{})[()[]]]}}>><[[{{[()()]({}())}}({(()[])[[]()]}{\n<<{(<(([{((<([()()]){{[]<>}{{}()}}>)<<<({}<>)[<>{}]><({}[])<[]<>>>>(<[[]]{(){}}>(([][])([])))>)[{[{([]<>)[<>{\n[(<(<<{{[([[[({}{})<()())]({[]}{{}[]})]({[()()]([]{})}{({}[])[[]()]})]<[[{<><>}<[][]>][[[]()]{(){}}]]>){(([<(\n[[<[<{<[{({{{{<>{}}<[]()>}<<(){}>>}<{{{}{}}{()<>}}[([]{}){<>{}})>})[[((<<><>>[{}[]])<{()<>}<()\n((<(<<[[{{{(<<{}()>[()()]><((){})>)[[[[][]][()[]]][{[]<>}<{}{}>]])((([[]{}]<[]{}>))([{<>{}}(<\n{{(({[{[({({<<<>[]><<>[]>>{{()[]}[{}()]}})[{(<{}<>><[]>)(({}[]){{}[]})}<<<[]()><[]{}>><{<><>}<()<>>>>]})\n[(<[{[<({{[{<[<>()]([]{})><{(){})[()[]]>}<(<()()>(()()))(<{}[]>({}[]))>]([([[]()]({}{}))({(){}}([](\n{({(({[<{<[<<{<><>}(<>[])>[<{}{}><[]<>>]><<{<>}>>](<<<()<>>(()<>)>{<{}[]>([]<>)>>)>}<{<{{<<\n<([[[(({[{<(({<>[]}<<>[]>){<{}[]>([]()))){<({}<>)<[][]>>{<{}{}>{()[]}}}>{((({}{})[{}[]]))({[<><>]}<<<>{}\n[{[[[{<<(({[({[][]}<[]<>>)<{{}()}{{}}>]{({[]}[<>{}]){<[][]>[{}{}]}}})[[([<<>()>(<>[])]<<<>\n<({<<[(({[([{[()[]]({}{})}{{()[]}[()[]]}])[([((){}){<><>}]{<()[]>{{}{}}})]][[{<[()()]({}<>\n[{<([<({{<<[((<>[])(<><>))((()[])([][]))][(([][])<<>()>)]>[<<{()<>}<{}<>>><<{}<>>{()<>}]><{[()[\n[<{[<([{((({<<[]{}><{}{}>>([{}<>]<<><>>)})[[(<{}<>>([]<>))<<{}()>{{}<>}>]{[<[]<>>][[{}<>]<{}[\n({<{<[<({([<{<()()>{{}[]}}({<>{}})><{[[]<>]<[]()>}<[{}()]{{}[]}>>])}([<<[{[]<>}<<><>>][<[]{\n{[({<[[<<<(<{<{}<>>(()())}[{()}{()<>}]>)[<{{(){}}(()<>)}>{[([]{})(()<>)]{[(){}]{()[]}}}]>{{[<<[]()><()<>>>{\n<[{{({({(<<{[(()<>){[]()}][<[]>(()())]}{<(<><>}{()[]}>}>(<<{{}()}<[][]>>[{()<>}{{}{}}]>)>)}<[(<((<(\n[<{[<[{[[[([<(<>{}){{}<>}>(<<>{}>{{}{}})]{({<>[]}<{}()>)<({}<>)<{}{}>>}){<[[[]{}]{{}{}}]<<[]<>>(()[])>\n{[{((<(<[(((<({}[]){[]()}>)))]([{<(<<>{}>[[]])<[[][]][(){}]>>[{[{}<>]{[]<>}}{(<>())(())}]}[{<{{}()}<<>[]>>}[[\n[[[{{(<<<<((<((){}){[]<>}><<(){}><()<>>>){[<[][]>({}[])](({}{})<[][]>)})[[{(<>>}([{}()][{}()])]]>>>((<{[\n[[({<[[<([<[((<>{})({}<>))({{}{}}(()[]))]<<((){})<()()>>[<[][]><{}{}>]>>({[<()()>[[][]]]})])>\n([<(<{(<<[<{{([]<>)[<>[]]}<<()()>[(){}]>}><([[<>[]]](({}[])))({(()())<(){}>}<[<>>[()()]>)>]>>)\n{(({([{[{(<[[((){})(<>{})]<(()())(()[]>>]{([{}]{[][]})}>)<((((()<>)[<><>])<((){})[()()]>)[[<()<>>][<{}\n{{([{{<<({{<<<[]()><()[]>}{{()<>}}>[<{()<>}<[]{}>><{{}[]}[{}[]]>]}})({{({[<>{}]<(){}>}(<<>>(()<>)))(<\n<[[{<(<{{<[<{({}())}({[][]}{<><>})>{[<<>{}><()[]>]<[<>[]]<<>[]]>}]>(({[{[]()}<[]<>>]{<<>><<>[]>}}))}([[<\n([{<[{<({{[((<{}<>>[(){}])({()<>})){{<<>{}>}[<{}()>{()[]}]}]}[{(({()[]}(<><>))[<{}()>(<>[])])<[{()[]}[\n{<[{({<((<({[{[][]}{{}[]}]{[<>()][<>{}]}}((({}())([]()))<([][])[[][]]>)}({[[[][]]({}())]}{({{}}){(<>[])\n{{[{{({(<(<[[[[]{}](<><>)]([[]{}]{<>[]})]>({{{()[]}([])}[{[][]}(<><>)]](({[]}<<>[]>))))>)}({{[{{(([])\n[<[{[{<<({{[{[<>{}]<(){}>}]}<{(({}()){{}<>})(<{}()>{[]<>})}{<[[]{}]>}>}[<[[([][])[[]]]]{{([]())[{}<>]}<({\n([<({{[((<({(([]<>){[]<>))[({}())([]<>)]})[{{{{}<>}{{}[]}}}<<([]{})({}[])>[<()[]>]>]><([[[{}<>]<()>]([<>{}\n<[<{[[[<{{[[([[]<>]{(){}))[[{}<>][[][]]]](([<>[]]<<>[]>)[(()<>){()<>}])]}[<<((())(<>{}))<{[]()}<[\n[{[[{[[(<<(<({{}[])[<>()])><[[[]()](<>())]<{[]<>}({}())>>){[<{<>{}}[[]<>]>]}>{{<[<<>()>[()[]]\n[(([{(([{<[<[<{}[]>[<><>]]>({[{}{}]{[]{}}})]>}{{[[<[{}{})<()()>>{{<>}{{}[]}}]{{([][])[<>{}]}{\n(((<<<<[(<[<{{()()}<[]{}>}<{[][]}{<>[]}>>[[[()()]{{}{}}]]]({[<()()><[]{}>]{{[]{}}([]())}}<{<()<>>[<>]}<<{}\n<([<[[<<[(({(({}())<{}>)<(()[])[{}[]]>})<<({{}}[[]{}])[({}{})((){})]><<{{}()}{[][]}>>>)]>[[(\n<<<{(<{{(([{<{()[]}><<[]{}>{{}[]}>}{(<[][]>[[]<>])[{()<>}]}](<[[<><>][<>[]]]([()[]]<{}()>)>))<(\n{[[([<<[[(<((<()()>(<><>))[[{}()]<()<>>]){[((){}){[]{}}][[{}[]]]}>([<((){}){(){}}>[([]{}){()[]}]]))]<[<<{<<><\n([{[{([<{{{[<(()[])({}())>{[<>{}]{{}<>}}]{(([]<>){<>}){[()<>]<()[]>}}}(<<[{}<>](()[])>{[[]()]\n[<((<<<[{[((([<>{}]){[(){}]})<<{<>{}}({}[])>[[[]{}]<()[]>]>){<(<<>()>(<>[]))[(()[]){[]<>}]>\n<(([({[[[[<<[[[]()]]<<()[]>>>>{{<[()[]]{{}{}}>{<[]()>{()[]}}}<[({}<>)[()<>]]>}]([([[()()]<<>{}>])<{<{\n<{([([((<{{[([()[]]{{}()})][(<<><>>){{{}[]}[[]<>]}]}}>[((<(<<>>[[]<>])[(()[]){()<>]]><<{{}\n{<[[({{<<[<<<{()[]}<<>{}>>([[]<>]((){}))>[{[<>()]{{}<>}}[({}())]]>{[{{[]()}({}())>]({{(){}}<()<>>}{<(){}>{\n{(<[{[{{[<(<({(){}}<[]()>)(<<>[]>{<>{}})>)>]({[{[({}()){<><>}]<[[]<>]{<>{}}>}(<<{}{}>([]<>)>(({}{}){(\n((<((((<<({[{{{}[]}(()())}{<()()>}]<[[<>[]][(){}]][{()[]}]>}(([<[][]>][<[]()>({}[])])[<<[]()>[{}<>\n[[{<[(<{{<{<[{<>()}[[]]]({{}<>})>{{{()[]}<{}<>>}}}<({{()[]}}[[<>()](()[])])<[([]<>)[{}<>]][([][]\n<{{[{[{(<<({{{{}[]}{[]<>}}<[[]{}]{{}[]}>}[({{}{}}<[]()>){[[]<>]}])((<{()()}[[]{}]>({()<>}<\n({{{<({{<<[<[{()<>}[[]<>]]<[{}{}][<>[]]>>({[{}()]<<>>}[(<><>)({}())])][([{<>()}{<>{}}]{(<>()){()[]\n[((<({(<<({[<(<>{})<[]<>>>[([])[(){}]]][{{()<>}(<><>)]<[{}{}][{}<>]>]}{<{(<><>)[{}[]]}(([])[[]{\n(([{<[<{<<{{({()[]}{<>{}})[[[]<>]<[][]>]}<{<()<>>[()()]}{<<>><()<>>}>}>>[<[[<[{}<>]{[]{}}><(<\n(<<<[<<<(([{{<()<>>[<>[]]}{<()[]>([][])}}])(([{{{}{}}[<>()]}({<>()}{[]{}})]<[<<>[]>]{({}<>){[]<>}\n{<<[<([[({({{<{}[]>([]{})}[<()<>>[(){}]]}{[(()<>)]{<()[]>({}[])}}){<([{}[]]({}<>))(({}{})(()<>))><([(){}]<{}\n{{[[<(<({<([[{[]{}}(()[])]<(<>[])>])({[[{}[]]({}<>)]})>[(<<[{}{}]<[]<>>>>{{{[]<>}[{}()]}({<><>}<<>()])}){([<\n{[[{{<(({(([[<()>{()<>}]([[]()]([][]))]<<({}[]){[][]}>([<><>]{()()})>)((<[<><>]{[]<>}>[[[][])[[]<>]\n[[<<{[[{([<{({[]()}({}())){{{}<>}<()()>}}([{{}()}<{}()>][{<>}[()<>]])><{{{{}[]}(<><>)}{[[]<>]<[]<>>}}>])}\n({(({(<({[{((<{}><{}>){<{}()>[{}[]]})}]{[<({<><>}({}()))>{{({}<>)(()<>)}{<{}{}>{<>[]}}}]}}({[{([[]{}])[<()\n({[{<<[<<[{{{{[][]}[()[]]}<<{}()>{{}()}>}{({<><>}([]))[<{}<>>(<>{})]}}{[([<>{}][()<>])([()[]])]}]<{[(<<><>\n{<{<(({[[{(({{()<>}[[]<>]}[[()<>][()()]])(<((){})[<><>]>{[[]<>][()<>}}))}<[<{{{}}[{}[]]}<(\n{{<[[(<[{<<((<{}<>>{(){}}))((<[]<>><{}{}>)({<>()}<()<>>))>>({[{(()[])<{}<>}}(<()()>)][<[<>{}]{<>\n<[<{((([<(<<((<>())<<>()>)>(<{<>()}{[]<>]>)>)[[{{{()[]}(()())}{({}())({}<>)}}](<[{()()}<()()>]((()){<>[]})><[\n(<([{[[[[[(<(<<>{}>)<{{}{}}>>(<<[]()><[]()>>(<(){}]{()()})))]]]({(<{(<()>([]{}))[{()<>}{[][]}]}([<\n(([({{{<(<{[<[<>[]](<>())>(<{}{}>)]<(([]<>>(()[]))(<()()>(<>{}))>}>({{[{[][]}][{[]<>}([]{})]}(<(()\n({({<{[{{([(<(()())><{[]{}}(<>())>)(<[()()]{()()}>[[[]()](<><>)])]{<<[[]<>](<>())>(({}{})<<><>>)>\n((([[{[<[{(<[{<><>}{{}<>})>{<[<>{}](<>)>[<[][]><[]{}>]}){[{{()[]}<<><>>}<(<><>)({}{})>]{<{(\n[[<({([{{[<{<(()())<[]<>>>{[[]{}]}}(<[{}<>]<<>()>><(()<>)[()()]>)>>{<{<[[]<>]{[]()}>{<[]<>>{[\n(({[[(([[<{([[{}<>]{()()}][[{}[]]{[]{}}]){[[()[]]<[][]>][<{}{}><{}{}>]}}[{[({}())[[]<>]]}{[({})<<>>]<{<>[]}";

static BRACKET_MAPPINGS: phf::Map<char, char> = phf_map! {
    '[' => ']',
    '(' => ')',
    '{' => '}',
    '<' => '>',
};

fn part1() {
    let x: Vec<i32> = INPUT.lines()
        .map(|l| find_illegal_char(l.chars()))
        .filter(|ic| *ic != ' ')
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!()
        })
        .collect();

    println!("{:?}", x.iter().sum::<i32>());
}

fn find_illegal_char(chunk: Chars) -> char {
    let mut stack = Vec::new();

    let mut illegal_char: char = ' ';

    for x in chunk {
        println!("x: {} --- stack: {:?}", x, stack);

        if vec!['(', '[', '{', '<'].contains(&x) {
            stack.push(x);
        } else if *BRACKET_MAPPINGS.get(stack.last().unwrap_or(&' ')).unwrap_or(&' ') == x {
            stack.pop();
        } else {
            illegal_char = x;
            break;
        }
    }

    illegal_char
}

fn get_remaining(chunk: Chars) -> Vec<char> {
    let mut stack = Vec::new();

    let mut illegal_char: char = ' ';

    for x in chunk {
        if vec!['(', '[', '{', '<'].contains(&x) {
            stack.push(x);
        } else if *BRACKET_MAPPINGS.get(stack.last().unwrap_or(&' ')).unwrap_or(&' ') == x {
            stack.pop();
        } else {
            illegal_char = x;
            panic!()
        }
    }

    println!("{:?}", stack);
    let ret = stack.iter().rev().map(|c| *BRACKET_MAPPINGS.get(c).unwrap()).collect();
    println!(" --> {:?}", ret);
    ret
}

fn part2() {
    let x: Vec<&str> = INPUT.lines()
        .map(|l| {
            if find_illegal_char(l.chars()) == ' ' {
                l
            } else {
                ""
            }
        })
        .filter(|ic| !ic.is_empty())
        .collect();

    println!("{:?}", x);

    let xx: Vec<String> = x.iter()
        .map(|y| get_remaining(y.chars()))
        .map(|y| y.iter().map(|c| c.to_string()).collect::<Vec<String>>().concat())
        .collect();

    println!("{:?}", xx);

    let mut x1: Vec<u64> = xx.iter()
        .map(|xxx| calc_count(xxx))
        .collect();

    println!("{:?}", x1);

    x1.sort();

    println!("{}", (x1[x1.len() / 2usize]));



}

fn calc_count(s: &String) -> u64 {
    let mut score = 0;

    for x in s.chars() {
        score = score * 5;
        println!("{}", x);
        match x {
            ')' => score = score + 1,
            ']' => score = score + 2,
            '}' => score = score + 3,
            '>' => score = score + 4,
            _ => panic!()
        }
    }

    score
}

fn main() {
    part2();
}