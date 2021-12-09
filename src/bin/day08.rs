use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use itertools::Itertools;

const E1: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
const E2: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
const INPUT: &str = "bfdcga cgdfe egfda cbfeda ec efc gcfbd ebcg gdfbec adcgfeb | bgdfc ec dfgae gefcd\ncafgbd bc gfdbae eacfb cbf debaf cdgefba fecag fbcdae ecbd | cdfabg cb adgfcb bface\nafg daecgf aefbgdc fbge fg dabfg efabgd cbadge abcfd gbaed | acbdf egbda adbge fg\ncbeafg fadce cdeabfg gbdefc ged ebcgf bfgd dg ecdgf bgedac | fgdb afcbeg gcfeba edg\nefd aegdb efgdcb bafcd acef dfaeb ef fgbadec cadfgb bfecda | fedgcb begcfd def gfdbac\nbadce gbcda gad fgdc acbgf cfbage gcfbad gd dbafeg afcbged | ebdfag dfcbag gd fdeagb\ngdbcaf da dcbeg cad edcbfg gdea baedc afbec baegdc gbfaedc | gbdcaf dbgec da gaed\nbedga agbefd dbgfca gcafdbe begcdf gdabf abegc deb ed adef | bgade gacbe ebd cbefdg\neg edfcab gfde abedf fbgdea bgcad gefcba agbed ebg eafbcgd | ge fbdeca bdcfega cgadb\ndgbaecf fegbd de fbgade fde gecfb ebda dbgfa dfegca dfagbc | afcegd fgdabce de efgdac\ndgafe deafbg gdecbfa fgcad beagd gfbcae efa dbfe ef dcgeba | gafedb debacg ef dfbe\ngbadcf bcade efdgbac bcf acebf bf egcfa ebdf acbgde cdabfe | fbcea eafcg fcbea dcbafge\ndcfbge ecdabg gfbdae afecb cbg cbgae gacedbf gadc cg bedag | bgdae becga bdcgfe ebcgda\nbde eb cgbfade cdgbef beag gabdf dafeb cadfe agdfcb eadgfb | begfdc dfbcga abfdg edagbf\ndcbgea fced cgeaf cfaged gdeac dgbfca fc badgfce cfg egbfa | fbcdag feagb cdef fcg\ngfdbc adebcg cbg dfcaeb cg bcaefdg bdgfec bdagf dcfeb egcf | fbgda aedfcb cfgbde defgcb\ndaf cagfeb adeg dcbef cdgbaf da efcag fbdegca ecafd dcaegf | da bgeafc bgdfac ecgdaf\ngfdaeb gfde agebf bgf fg bgcfad afbde fdaecb dbeacfg eabcg | gecab baedf eadfb gcbdfa\nbagcfed eafb acfgd cebdfg ead agbdef baedcg egfbd agefd ea | gacfd bgfed fgdae bfdeg\ncgfab adcfebg cgdef eabc feagbd ebcfg feb eb bdfcga gbcfae | defgc bdcafg fgdec cegbf\ncdfab gdf cfebg decbaf gd dcgbaf gcbfd cgad egdbaf badcfeg | gfcbd gd cbgfe ecbafd\nfaebd ba efacgd baedcf adcef abf dcfbgae dbac gdbfe begcaf | dcab dafec fecadg ebdfa\ndfceab egbadcf ba fdcbg feba cgdfae dbegac abdfc cba ecfad | defacg dgbcf gbdcf efcdbga\ncadgb bedg fgaceb eacbgd egbac cefadg gd cafdb eabgdfc gad | afgdce adcgb gbacd adg\nacb adcgf adcfeb baedg cb becagd bfcadge abcdg ebgc gfdeba | gdafc acfdeb cb aebgd\nfabcgd fcdgae facde bcafeg da geda gcebadf befcd eagfc adf | cfage fgeac fgdbaec fgeac\nbafeg fabcdg efdgabc fgdbea gdae ae dgafb bae ebgcf fcadbe | begcf abgfe aegbf efbadc\nfdeca cadegbf gce bafdge abcdeg dcgb ebcfga cg deagb dagce | bdeag aebdgf gce dbage\nfcgbde cdgaebf dgbaef fed bade ed adfgb efdag fdabcg agefc | gfdab adbcfg gcfae dfagb\nebdgfc febad adbfc fc bdfcea badfge cfb eafc acdgb egcfdba | acbgd bgfead dfabc fcea\neb ebaf cbdgef gaedb afgebdc febdag dgefac agbcd beg deagf | dgfcea efabdg egabd geafd\ncedfba efgbd gfbdac gfaecd bfa gdfba bcag ab cgdfa begdafc | dfbace dfgeb eafcdbg dfcag\ncfdbge gdbefac dabf dgbcfa gacdf gebca fb agefdc bcgaf fbc | afbd gbcafed acgeb ebgfdc\nfgbec fbgcd fdbaceg gafedc bfacge fge eafb agbced cbgea fe | abgce fadegc ebacg fagecd\nbdgec bgecfa fcbed debfcg acebfd gcb adbecgf gb aegcd gbdf | ebgfdc geacbfd gcb gb\nfbea bdfcag dceabg egfcd edcfbga abdge af adf dfbage egfad | fa deacgb agfdbce ebdag\nabdecfg gbacf fdbcge gfd dg gdafbc adgb acfde fadgc fbeagc | dcgebf dacgf agfbc cdagbf\nagcefd ged bdfagc ed ebdgf fcdbge dgfbc abegf cdfegab debc | cagdfe ged cgdafb bafdcg\ncebgad fdgea abgf fdbcae af baedg dgfeba dfbagce edgfc fea | dabegcf agdbe afe acfedb\nagdfe fedbcga gfedbc agcbe edb db fdba edbgaf gedafc abdeg | cdgabfe afegbd fegbcd eafgd\ndea abgdfe edgfbac egdab de cadbef adfbg gbcea gfde dabgfc | cdafeb ed bafgde ed\nadgfb cdgefa ea fea bcdef cfagbd fgecabd febda agbe fgabed | cdefb gbea bdacfg dcbef\ncadbg ea fdgeab afcedg dae ebcfdga cdfge afec dbfgce ceagd | fbeacdg dgcab bgdca dae\necdbfg caebd fcgde fgda aefbcg afedc fcabegd aef gcdaef af | ecdfa bdecfg fea dgcfbea\ndbcagf afgec fcadb ead daefc dbcgae edfb cagbdfe fcdbae de | egdabc fdagbc cgafbd afdec\ndgfabe egdcbfa beacdg dbcfg aefc cdbaf fbadec fa aedbc fab | eacf bcdfg efca ecbad\ndefbgc gbdef cg edgc egbadfc cbgef gafdeb gcf eacbf gbfdca | fbedg decg cgf egfcb\ncfdgae abdgf gbcd cfdaeb afebcgd cafgd bdf bcfgad db gefba | egfba decgfa abfeg eafdcg\ndfagb gdbface adefbc gbfae gae begc acebgf fdgeca ge cefba | cgefab dfgace fecagd beadfc\nfedacgb ed cabdfg bfgade dbaefc gdfba def gead bgcfe bdfeg | adcefb gdbaf egad caebdf\nagfbed ecfbg cdge cgb fcgbde ebcaf adfbcg dcegabf cg efdgb | bdfcga bfeadg ecgfb cg\nbec becfdag bgcaf eb debf adcef cefba fbcade acbged fcadge | edacfb fdeca fgabc cgafb\nbfcae gabc fegcdb adefgbc bcgfae gfb fbgea fdabec adgfe bg | dfcgeb fgbae fgb bedgfc\nafedcbg ed dfagc gefcd dec agbdec bfecg gfedca efad dfabcg | edaf dec begadc dfagbc\ncgebd befcadg ecg ec fcde dbfgc fgdabc becfga aegbd bfcged | cbgafd bcgfd fedc gfdbec\ndfabeg acfebd bgfcde begfd bcagd cdafgbe gabed aeb fgae ea | gbedf gdebfa bfegcd bgefd\ngdf dagec gf cfdbega cfdba edbcga ceadfg fcdag aegf cdbfge | eafg fgd dfg bdegcf\nbcfae fabge cdegabf cdae ebc dfcba ecbdfg ce dagcfb fbedca | fdagcb bcedfa fceabdg adce\ncgbaf ad fgcadeb cbfdae fbgdac bgda egdfc cefbag adc fcadg | cfdgbae adc ecafgb dgabfc\nbafdc cfb cf abcfged acdbge dbgca aefdb afcg afcdgb gbefdc | dabfc cagf efbcgd abecdgf\nde abgefdc fbed adbcgf gaecf eda aebdgf adbecg fegda bfagd | feagbd dbgfac ade adgfe\ngacefb faebcgd decfgb fa abdgfe afb fdbge dacbg adfgb edfa | dgbac abcgd gbadc adbgf\nfbadg fgbde fagebd agbcf gad gfcbed abde da cbdfaeg afegdc | ecfbdg agd efdgcba aefbdg\ncbdge gfecd cfe cafgd dbfe fe bgefdca gedfcb gebfac dgbcea | dgecb bagcfde gecbaf gacbef\ndfgecb agbfe dbeacf ebcdf cbad cadfbeg ca cfa abcef edfcga | afgdbce cabd fcbdae becaf\nabcd fbade befdcag da fecdag ecfdab bgfced bfedc dae gbaef | da ebfdc cabd cfedbg\necdab cfaged cbfd cfebad begda agbfce fecba cd acd fedcbga | cfdage daecb febca ebacf\nfcdageb beadf dag dacfbg bacge deabg dfcabe gdbaef degf gd | bafcdg dgafbc bdaef gd\nfebcg cefbagd fegdbc bfeac ab gcba bfa fdbega cedfa cgfeab | eagbcf cbgef afb fcgbae\negfac acbdge fadcbg bgcedfa bdcafe bf ecadb fdeb fecba bfc | fgdcabe dcbfga aegcf fbcadeg\ngcfebd acdgb faedcb gcdfb fd gdfe fcgbe fgacdeb baefcg dbf | cgfdb bgdcf gabcd edcbgf\naced fbacg adbcf cd fbade cdb decfbg faegdcb gedabf ecdfba | bdfae fdgcbae fdbgec fdecgb\ngdbeac dfcaeb afbgecd dbcgf af fbacg cegbfa fba cagbe geaf | fgbdcae eabcg caebg afeg\nebfda dgfbe ae fcbgead deafgc fdbac eaf gaeb ecdbgf ebfgad | baedf debgfca efcbdg ea\ndgcfe acbef cgaef fbdgace ecafbd gcebfa ga age gdfeba bgac | adfbce ga cegfa abcg\nabcgd facd fag eacfbg gafdb fa bgfacd dcgeba befgd efgacbd | bdcegfa gbdaf cbgda egabcfd\nbdefcga aecbg cgde agbed ecabfg dgb befda gcadbe gdbcfa dg | acgdeb dgb bfaed adbeg\ncefgb bde adgceb dafgebc aefd debgaf gfedb agfdb de cgdfba | defa edbgf afbdg bgfad\nfa caefg egcab fca gdbcea abef bceagdf dcfge acgfdb afbgec | abfe faebcg acgef geacf\ncbfed egacdb eabfgd gdbac deacgfb aeb eacdb gacfbd ea cage | dbegacf aecdbg cage dcgbae\nefgad agedcbf bcea ba ebdcfg efbcag bgaef gfabcd bga gfbce | fgdae ba fedgbc egfba\ndbcfa bcagd gdbfae cdfg fecab bcfadg dcbgae dfb fd bcadfeg | daegbc befca cbfae dbgcae\negfa bcgdafe dcfea dcfeba cafgbd gf edgbc gfd egfdc edcgaf | gf dgafce fdg efcbda\nfgecd afgcde gfcadeb dbcg dcfbeg facbe gbf gfbec gdeafb gb | abdgfe cegdf bgf ecgfbad\ngdcbafe gaf agcfe bafc gfdcbe gbfdea cfegb gaedc gfbace fa | bfeacg acgfe bcaf gcade\negbcf fgcea decbf dgabef gb fabedc fbecdag fgdbec bcdg beg | bgdfaec acgdbfe defbca fcdeb\nbdeafc cabegd acdbe fdba cfb bf edcfagb fecag cbafe bcfgde | bfda fdgaebc dbaec fb\nbc ebdcg agcfebd cadeg ecb fdbaec befgd cbdgfe gabfed bgfc | dagbef bcgdfe bgedaf gdace\neb ebf gaedfc fagdb cabe fabecg bagef bdgafce cgdefb efcga | beacfgd efgbca efbgca ebfgac\ncagdf gdacef fcdbga cb gcadb bgc fcab dbfcage baged gbdecf | aefcdg cb agdfc bdgfec\ndfbaec abegdf daf edbgcaf bgfd eafgc gafed fd bedag gebdca | ecfgbda beafdgc facebd bdeag\nbfaegd dbecga fcadbg dcfg dabcg gf acbfe fgbca fga aegdfcb | cegbda ecabdgf egcafdb fgcd\ndbgcea agc aegbc fgceda abdfce cbdg begaf adbcefg cbade gc | dfecba gbedac cdfeab abecg\ncfdeab fgbec eadg bga ga dcebagf beadgc gaebc bgfdca bcead | bgaced bdeacg cbgea beacgdf\nbagfce ebcdfg fa cgade cfa bfcgeda ecbdf adfb fecbad efadc | ecdbf cfa bfceag bdfeac\naf egdaf aef eacdg fdgeca cafg cdgfabe baedgc edfbg acbdef | edfga decagb bdecfa dacfge\ndab gfbed bfdage ebdcgf baecg da dfacbeg eagbd beacfd gadf | egbda cbdfae cfadeb egabfd\nbcgfaed bfgade bd gfcda acgedb bacefg caebg cbed abdcg bdg | cebag fcebag dbcaeg gbd\ndgcfb begdf def fbaegd fegab ed cebagf cefdgab cebdfa gade | ebgdacf aefbgd de gfdbae\nab abdf agbcde aegfb eafgc fgcdeb fbged bea bgafced efdbga | gbafe gbafed gaefdcb gdfbec\neafbg abgdf eg aceg gabfce fge fcbedg ecfabd bafdceg caebf | cedbfa gbface eg fagbe\nface bfe adgfeb acebg cdbfg fe gacbdfe gcaedb cgebf cgaefb | aefc gdbcf gecab gfcbd\nagfecd fa gbfa fbdce gdfeacb dabfe fbadeg bgdaec begad afd | fda bdefc egadbf fa\nab bfedcg gfdcb deacg abdfce gcabd adcgfb eabfcdg abc abgf | decga debfac fcbdg gecdbaf\ngfadcb dcgab abfgde fdcega acd adbgf cgdeb ac acfb befdcag | dbagf ca gdcfab gaebcdf\nefcagd dbcef bgda gbf dfaegb cfegab bfged bg agcfbde afegd | bfagce cefdb fgeda aegfcd\ncedafgb cbfeda bcegf gdbe gb fcgbad cedbgf gcaef bgf cefdb | ceagf dbfcge afdcbe ebdfgc\ncgfed acfge bdecaf fdag fecgad fedgbc bacge af efa afegcbd | cgadbef gbdecf fgced fa\nabegf fagcde ceb feabc ecfda cb acdb efcgdab cbgdfe ecadbf | eafcb ceb cadb bdeafc\nfbgae begd abcfegd bfeda efd eagfdb de fgaecb acgdef dacbf | dagfce adefb bdfac dfegab\nfd fed fgad dfecag gaedbfc gecafb bdceg cbdafe efcgd fcega | agfd dgaf gdafce eafcdb\nbefagd dfeb bf caebdg ebgda egfcba bgf gcadf gafbd fabgdec | agdefb eagcfb edacbg badfg\neagcd bdacgf gfeb fdgba be deb cefagdb afcdbe bdgae aegbfd | cfbgad dbage eb bgeadf\neacbfg debgac dbfcage dc eagbc cda cabdfe dfgea bgdc adegc | becag efbacg caebg cdgae\nfed fcbagd acdebf gaef ef fgdec fecbagd dcegaf cbdeg agcfd | bdeacfg ef cadgf dcbeg\nefc becdag afedc edgaf fc badgcfe fcdbge adfcbe cafb eabdc | dfebca acbed fc efacd\nabfed dc fdcegb cgbfe cagfeb fdecb gbcadfe bgadcf gcde cdf | gecafb fbegdc febcdg cegafb\nfeagc daecfbg gdca dg fgd fdgec cebdf eagcfb efcgda fbdeag | defagb aedbfg gfd faegbc\ncagdf ce bcegdf dbafeg cbeadf gbdaefc caeb cfe feadb efadc | ecba dcfae ec ce\nfgabd abgdc dbcae cfadge cag cbge cg ebcadg cgfdeba fcbeda | dgabce dacegf baecd cfabed\nefagc fdcgea geb bfdgec cgba fdcbgae adbfe bg bfcaeg fgeba | afegb daebf ceagdf agbc\necgfbad ceg deca gbfed bagfdc cegfba cabdg ec agcdbe bdegc | agbcd ecbdg bcadg dcagb\naedbg bedca efbdag dgcb bfeac efgadc abegdc bfdegca cd edc | cbdae edcgfba ecd fdagbe\ncfeagbd cfab bgc fbgdae fgdce gfadb bc ecadgb bcdfga gdbcf | cbfa cbadgf bfcdg abcf\nedacg cdfbag fbeagdc cdgaef fc bagfe fac edabgc defc gcefa | dagcfb aegcf fc fegac\ngfdabc dbgaef agbe cegadfb bdgaf gedafc fbced ge gfe befdg | dfcbga adbfge bdfeg cfadge\nbcafge gbcd bc bfgdea cafdb fabdg acfdbg acefdgb acfde bfc | bfdac cbf fdabc edfca\ngdcab cegd ebdcfga egb eg afceb cdgbfa acbge gdebaf bgcdea | agbce fceab dagfbe acebg\nbagc cgf bacfd begfdc afdge bcagdf bcedfa cg becgafd dgfac | bdacf cfedab abcefd cafbd\nca cag gbfdc cadfbg eafcdg afcb edgab dgcbef cbgad abfdgce | cdbga bacf fgdace dcfgb\ndcebf aefcbd gdcf fcbgde cbgef beacg gfebda gf fcdbgae feg | bfcadge cfbeg fg becgf\ndcabfeg fbc cfgba dfabgc becadg fc dgebcf cfad cbagd gbeaf | ecadbg degabc aecbgd cf\ncba efabd ebdcga cgeafbd cdabf fbce dbfcae bc cgdfa egbafd | abedf facdg ebcf cb\ndb efdcag dabf cagfdbe caegb decba bfcdeg fcabed fedca bde | edfacb adefc dbe dbe\nfdabg efca bac ca fdacb becafdg fcdbe bcegad begdfc afbdce | gecdbf faec fadcb dbcaf\neg egad egfbadc bdecf feg agfcbe decfag dfceg cfgda cfbadg | fcabdg gef eg fecadg\nfgad bgfde afe acebd dgfcbe af beadf ecabgf fbcedga abefgd | cbdegf edfab fegacb fa\nefcgb beacfg gbecfda bfac afg dbfgce bfgdea fagce caedg af | gcebf aedfbg eafcbg fabecgd\nbfdecga gaedfc ecga begcfd cfdab gbfade efa caedf ae defcg | daecf egca afcdeg caefd\ndecab efa dagefbc dagbce fcadg abdegf fe eafdc ebcf dcfabe | ebcf fe ebcf fae\nbegfd agedfc dabfce eadgf adf eacgfb afecbdg da feacg dcag | abcdfe daf debgf dbfeac\ngbdfca gbefca cba cb ebcfagd gbafd fdacb fdgeab bcdg cadef | afbdc efdgabc dbagf gfdab\nefdagb dae fcabedg caedb fdaecb ad ebdfc cdgfeb dfca eacbg | bedfgc fecdb edgfba cedfab\ncd afdc cdg cbdgaf eagbd gacfb befdcga bdacg cbeafg ecfbgd | bdgacf dcfbeg dacgbf dagcb\nbegdaf gedbcfa dafc fea dfgce dgacfe gbedfc cfeag cbgea af | gecba agfbedc cbage egfdc\ndc fgcd baefcg afbgecd bgeda ced baedfc cdeag gdafec efgca | bacfge bgdae abegfc afgbced\nfc afcbgd bcf afebcgd bgdcf acdf bdfeg ecbagf cbdega adcgb | ceagbf befdcga edfagcb acdf\ndbcgf degc bfgad bcfed gcb fceabdg gbfcde gc fbdaec afgebc | fecbdag fadgecb afbdg dgecfb\nabcfg cfb eafgb bc cgbfda adfcge afdcg cfbgde cadb efgbcda | acgfb cb dfcag gdecfb\ncdefb cfga edcgbfa cab abfge acfbe ca bcgaed bcaefg bdegaf | dbfce gdfcaeb fedabg fbcea\nfgcae gafd eacdfgb baceg cadebf cfbged daefcg fge fg fceda | gceadbf dcbfage cafed ebacdf\nfabdceg abcgde acebdf cfaeb fa egfcb dbfa fac abecd agefcd | cfeab edcbag acf dcabefg\ndgbecfa abdfe acegd fdgabc gadef gfce ecbagd edagcf fg dfg | gdf fcegbda agcde fgd\ndagf bcdaef ad acd cbeag febagdc dgbfac dgcab fcdbge bdgcf | fdbeac gcbda bcfdg bfcgd\nfgdc gc gefdcab bdgec dbcefa gce bfgdec gcefab degab cebfd | bdfce gbecfd gadbe ebcadgf\nfagc bdcfag fdacb agbedf daf aedgbc dbcga ebdgcfa cbdfe fa | febdc cfag af cadbg\nbcd bfec dagbcf egcbfd bc edfgc bdecg edabg dcgeabf gdefca | bedag dbc fgcdbe ebdfcg\ngedafcb dbagcf bcadeg fc gfc cebf edgfc cgdbe bcfdge egdfa | fc dabgec bdcge ebfc\ngafbd ef becga ebgcaf agbef gfe gebcda gbfdce afce dbcgefa | ef gfe cabfeg gabecf\ndeab fbedgc eb bdacg dagcfbe abecg cfdagb beg baedcg gacfe | gacdb dbfgace dbea cadbg\nbadceg ced fecba fbdc adfec ebfcgad gecafb faedcb gfeda cd | fcead fbcaged adebgc ceabgd\nbd bdcg fgcab abd cfaebd baecgf fecbgad agdfb efdga gfdacb | afebcd bfadg egbfca cgbfea\nda decbf afdec begcad dcgefb gcafe edbfca dca fdab begdfca | efadc abdf ebcdfa fegac\ngcbeaf geadbc fdbeagc bfdec cdbae ea agfdcb dabcg eac adeg | gcdba cebdf abecd gedcab\nbcdae abedg ebg bfadec eg dbgaf egafcb gcadeb egdc abedfcg | dagfb ebg gebcaf eg\nafb abgfed caegfd cfbd becag dcbgfea bf decaf faecb bedfac | daegfb dbaefc gacedf bfa\nbgecf becfad ea adebgc cfdbgea dabcg eca eagd acbeg adfcgb | agdbc bagcd bgcda aedg\nbf fagcde cfdb acfgd gfdaeb acdfegb gfacb bfg bgeca bcgafd | gfb gfacd agfdeb abcge\naegfd efcd bedfagc bacdgf gecadf aed egbdac fcagd ed afbeg | daefg dagfc acdbfg gbacfd\ndecga efbcd badcgf cfgdbe agefcbd fa ebcdaf fcdea eafb fda | dcfea fad ebadgfc afedcb\nbf gebfcd bagfed bdagcfe cegbd fgbdc gfb ecgbad gdafc befc | fadbeg bcgde gbf bfg\ndf cbfged fgbca fbd bcafge bdeac agbfecd adcbf gdaf dabgfc | bcdea cafbd afdg gacfb\necdbfa fadgce aceb fgebd bfcgad cdfae afgbced dfebc cfb cb | cfead fcb bc bedcf\nfdbega cfegd cdbgae cdfebga fecadb aeg cbag ag adceb dcage | ebcagd gcead egdfc dagecb\ncdbfeag gfdeba edfgca cead geadf gac fcbdg afgcd cabegf ca | fecgad gefda dafcg fdgca\ncfgbae cgefb egdbfc dcg dfcbg ecbgfda badcf gfed edcgba gd | gd cbefdg adbceg badcf\nbf bgf debfg bgade egbfcd cebdafg agdfce fgced dcbfag fcbe | adcgfe fdgeac fb ebfc\ngefdc fdagcb gfdac bdafce caf ac daegcbf febgda cbga dbafg | dagbf ac faedcgb dfagb\necfgadb dbgacf abfegc gd cefad fcgde cefdgb fcbge gdeb dgf | efacd bcegdaf bdgacf gd\ncdfgb bdf fdcag bd cdba gbecf dfecga cfgebda gdefab dcbagf | begfc bcgfd gfbce afbgcd\nfbe facb decgaf afdce fb bcdeaf bagdfe ecdbg dfbce bgedcfa | abdgfe eafdc efdbca acfgde\nbgdcf cbaegd eabfdcg ceg eg aedcb deabfc dgea gfcaeb gecdb | eacgdb ge daebcf eg\negbdcaf gd cefabg dfge bcfagd afdegc caedg baecd gad caefg | gcedaf efgabc dag deagc\ncdabf bgcfde afedgc fcb gcba gfacd bdaef bdegfac cb bdfagc | bcf bcedfg gaecdf cdgeabf\ngf dcbfeg fcgdbea fge gedacb aefbg bgead gfad dfegba aebfc | aedgfb gf egf dfag\ngbeacfd gacbd dcgfbe fg fgd dbacge bdagcf dfcea gafb fgadc | deagcb gf degfbc edcgbaf\nce gdbcae debcf cfdabe eacfbdg acef fgdbc begadf ecd afdbe | dbefa abegdc cbdfg acfbed\nbegdfc gbdc gedaf dc edgfc cdf fgbec ecdafb ebgafc degcbfa | egdfa edgbfca cbefg fagcbe\nefcab dgbfc ag agdc cfagbde fbgeda fag acgdfb fcbga gbefcd | gacd gdac dcag gdac\nbdage bdaefg gbe eb edfb bgafec dgbecaf egadf fdaecg dbacg | geb afgdbe dfbegac efgcba\necgfa bec be ebfgc dgbcea adefgc egafbc egbafcd bafe gbfdc | aefb bce befa gfecab\nebgdc cbe ecdgab gacb gfced dbgea edbcfa fagcdeb gabfde bc | dgfec aebdgf abcdfeg fbaedc\ndaegbf adef dgcabe cafdgbe adcgfb dbegf ebgcf dbf ebadg fd | fcbge bgcef acfgdb efbgd\ncdbge cfe adbcef fedgcb gfebcda cedgf edagf gdbcae cf gcfb | cdbeg gcdfe fabgdce aecbfd\nbeacg dafge gbeadc fcba bgdeafc begfa bf eabcfg ebgcdf bfg | gaefb bcfged degcab egdaf\nacgefbd feb bf cdaef beadf gbaed abgced gabdef abfg gbfedc | begfda efcda bfe dgacbe\neacfgd fbdgce abfcge bcfea cafeg cagdbfe ebf debca bf fgba | fb fb acdbe cfdage\nbfec cedagf aef ef degba bagcef dgcfab fgeab fecdgba bacfg | cbgaef efacgd aegfb aef\ndbacfe edfc ec caebd agfebc cbe cbdag gfcbead bgfead bafde | deabc dfagbe cgbfea ec\ned dfgbec dfabg fbaec feacbg edafb aedc eadfcgb ebdacf dbe | deac bgeafc cebfdg fcdegb";


fn single_part1(line: &str) -> u32 {
    let both: (&str, &str) = line.split(" | ").collect_tuple().unwrap();

    let all_after: Vec<&str> = both.1.split(" ").collect();
    let x = all_after.into_iter()
        .filter(|s| {
            vec![2usize, 3, 4, 7].contains(&s.len())
        })
        .count();

    x as u32
}

fn part1() {
    let x: u32 = INPUT.lines()
        .map(|line| single_part1(line))
        .sum();

    println!("{}", x);
}

fn get_all_numbers(mut numbers: Vec<&str>) -> HashMap<Vec<char>, i32> {
    let (a, b, c, d, e, f, g) = solve_numbers(numbers);

    let mut number_map = HashMap::new();

    // 0
    let set0: Vec<char> =
        vec![a, b, c, e, f, g].into_iter().sorted().collect();
    number_map.insert(set0, 0);

    // 1
    let set1: Vec<char> =
        vec![c, f].into_iter().sorted().collect();
    number_map.insert(set1, 1);

    // 2
    let set2: Vec<char> =
        vec![a, c, d, e, g].into_iter().sorted().collect();
    number_map.insert(set2, 2);

    // 3
    let set3: Vec<char> =
        vec![a, c, d, f, g].into_iter().sorted().collect();
    number_map.insert(set3, 3);

    // 4
    let set4: Vec<char> =
        vec![b, c, d, f].into_iter().sorted().collect();
    number_map.insert(set4, 4);

    // 5
    let set5: Vec<char> =
        vec![a, b, d, f, g].into_iter().sorted().collect();
    number_map.insert(set5, 5);

    // 6
    let set6: Vec<char> =
        vec![a, b, d, e, f, g].into_iter().sorted().collect();
    number_map.insert(set6, 6);

    // 7
    let set7: Vec<char> =
        vec![a, c, f].into_iter().sorted().collect();
    number_map.insert(set7, 7);

    // 8
    let set8: Vec<char> =
        vec![a, b, c, d, e, f, g].into_iter().sorted().collect();
    number_map.insert(set8, 8);

    // 9
    let set9: Vec<char> =
        vec![a, b, c, d, f, g].into_iter().sorted().collect();
    number_map.insert(set9, 9);

    number_map
}

fn solve_numbers(mut numbers: Vec<&str>) -> (char, char, char, char, char, char, char) {
    let number1 = get_and_remove_number(&mut numbers, 2);
    let number4 = get_and_remove_number(&mut numbers, 4);
    let number7 = get_and_remove_number(&mut numbers, 3);
    let number8 = get_and_remove_number(&mut numbers, 7);

    let a: char = **number7.difference(&number1).collect::<HashSet<_>>()
        .iter().nth(0).unwrap();

    let mut c_f_possibilities = number7.clone();
    c_f_possibilities.remove(&a);

    let mut b_d_possibilities: HashSet<char> = number4.difference(&c_f_possibilities).map(|e| e.clone()).collect();

    let numbers_of_size_6 = get_all_of_size(&mut numbers, 6);
    let number9 = numbers_of_size_6.iter()
        .filter(|n|
            n.is_superset(&c_f_possibilities) && n.is_superset(&b_d_possibilities))
        .nth(0)
        .unwrap();

    let g: char = number9.difference(&c_f_possibilities)
        .map(|e| e.clone())
        .collect::<HashSet<char>>()
        .difference(&b_d_possibilities)
        .map(|e| e.clone())
        .collect::<HashSet<char>>()
        .difference(&vec![a].into_iter().collect::<HashSet<char>>().iter().map(|c| c.clone()).collect::<HashSet<char>>())
        .collect::<HashSet<&char>>()
        .iter().nth(0).unwrap().clone().clone();


    let all_of_size_5 = get_all_of_size(&mut numbers, 5);
    let number2: &HashSet<_> = all_of_size_5.iter()
        .filter(|n|
            n.contains(&a) && n.contains(&g)
                && (set_contains_exactly_one_of(&n,
                                                b_d_possibilities.iter().nth(0).unwrap(),
                                                b_d_possibilities.iter().nth(1).unwrap()))
                && set_contains_exactly_one_of(&n,
                                               c_f_possibilities.iter().nth(0).unwrap(),
                                               c_f_possibilities.iter().nth(1).unwrap()))
        .nth(0).unwrap();

    let e = number2.difference(&b_d_possibilities).map(|e| e.clone()).collect::<HashSet<_>>()
        .difference(&c_f_possibilities).map(|e| e.clone()).collect::<HashSet<_>>()
        .difference(&HashSet::from_iter(vec![a].into_iter())).map(|e| e.clone()).collect::<HashSet<_>>()
        .difference(&HashSet::from_iter(vec![g].into_iter())).map(|e| e.clone()).collect::<HashSet<_>>()
        .iter().nth(0).unwrap().clone();


    let number6 = numbers_of_size_6.iter()
        .filter(|n|
            n.contains(&a) && n.contains(&g) && n.contains(&e)
                && n.is_superset(&b_d_possibilities))
        .nth(0)
        .unwrap();

    let c = c_f_possibilities.difference(&number6)
        .into_iter().nth(0).unwrap().clone();
    // .collect();

    let mut c_f_possibilities_clone = c_f_possibilities.clone();
    c_f_possibilities_clone.remove(&c);
    let f = c_f_possibilities_clone.iter().nth(0).unwrap().clone();


    let b = b_d_possibilities.difference(&number2)
        .into_iter().nth(0).unwrap().clone();

    let mut b_d_possibilities_clone = b_d_possibilities.clone();
    b_d_possibilities_clone.remove(&b);
    let d = b_d_possibilities_clone.iter().nth(0).unwrap().clone();

    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);
    println!("f = {}", f);
    println!("g = {}", g);

    (a, b, c, d, e, f, g)
}

fn set_contains_exactly_one_of(set: &HashSet<char>, first_value: &char, second_value: &char) -> bool {
    (set.contains(first_value) && !set.contains(second_value)) ||
        (set.contains(second_value) && !set.contains(first_value))
}

fn get_all_of_size(numbers: &mut Vec<&str>, size: usize) -> Vec<HashSet<char>> {
    numbers.iter()
        .filter(|n| n.len() == size)
        .map(|n| HashSet::from_iter(n.chars()))
        .collect()
}

fn get_and_remove_number(numbers: &mut Vec<&str>, string_size_of_number: usize) -> HashSet<char> {
    let s_number = numbers.iter()
        .filter(|n| n.len() == string_size_of_number).nth(0).unwrap().clone();
    numbers.remove(numbers.iter().find_position(|n| n == &&s_number).unwrap().0);
    HashSet::from_iter(s_number.chars())
}

fn part2_line(before: &str, after: &str) -> i32 {
    let all_numbers = get_all_numbers(before.split(" ").collect());

    println!("all numbers = {:?}", all_numbers);

    let map: Vec<&i32> = after.split(" ")
        .map(|d| {
            println!("{}", d);
            all_numbers.get(&Vec::from(d).iter()
                .map(|c| *c as char)
                .sorted()
                .collect::<Vec<char>>())
                .unwrap()
        })
        .collect();

    let i = map[0] * 1000 + map[1] * 100 + map[2] * 10 + map[3];
    println!("{}", i);
    i
}

fn part2() {
    let lines: Vec<(&str, &str)> = INPUT.lines()
        .map(|line| line.split(" | ").collect_tuple().unwrap())
        .collect();

    let result: i32 = lines.iter()
        .map(|line| part2_line(line.0, line.1))
        .sum();

    println!("{:?}", result);
}

fn main() {
    part2();
}
