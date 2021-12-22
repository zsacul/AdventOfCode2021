use crate::tools;
use std::collections::HashMap;

fn count3d(hash:&HashMap<(i32,i32,i32),bool>,x:i32,y:i32,z:i32)->i32
{
    let mut cnt=0;
    
    for zz in -50..=50 {
    for yy in -50..=50 {
    for xx in -50..=50 {
        //if xx!=0 || yy!=0 || zz!=0 
        {
            if *hash.get(&(x+xx,y+yy,z+zz)).unwrap_or(&false)
            {
                cnt+=1;
            }
        }
    }}}

    cnt
}



fn simulate3d(next:&mut HashMap<(i32,i32,i32),bool>,x0:i32,x1:i32,y0:i32,y1:i32,z0:i32,z1:i32,v:bool)
{
    let mut x0 = x0;
    let mut y0 = y0;
    let mut z0 = z0;
    let mut x1 = x1;
    let mut y1 = y1;
    let mut z1 = z1;
    if x0< -50 {x0=-50;}
    if y0< -50 {y0=-50;}
    if z0< -50 {z0=-50;}
    if x1>  50 {x1= 50;}
    if y1>  50 {y1= 50;}
    if z1>  50 {z1= 50;}


    let (px,py,pz) = (0,0,0);
    for zz in z0..=z1 {
    for yy in y0..=y1 {
    for xx in x0..=x1 
    {
        let newp = (px+xx,py+yy,pz+zz);           
        next.insert(newp, v);            
    }}}
    
}



pub fn solve1(data:&[String])->i32
{
//    on x=-22..26,y=-27..20,z=-29..19
//    off x=-48..-32,y=26..41,z=-47..-37    
    let mut field  = HashMap::new();

    for d in data {
        let tt:Vec<&str> = d.split(',').collect();        
        let ok = d.contains("on");
        let x0: i32 = tools::i32_get_between(&tt[0],"=","..");
        let x1: i32 = tools::i32_get_between(&tt[0],"..", "");
        let y0: i32 = tools::i32_get_between(&tt[1],"=","..");
        let y1: i32 = tools::i32_get_between(&tt[1],"..", "");
        let z0: i32 = tools::i32_get_between(&tt[2],"=","..");
        let z1: i32 = tools::i32_get_between(&tt[2],"..", "");

        
        println!("{}",ok);
        println!("x {}..{}",x0,x1);
        println!("y {}..{}",y0,y1);
        println!("z {}..{}",z0,z1);
        simulate3d(&mut field, x0, x1, y0, y1, z0, z1, ok);
    }
    
    //field.values().map(|f| if *f {1} else {0}).sum()

    count3d(&field,0,0,0)
    
}

pub fn solve2(data:&Vec<String>)->i32
{
    /*
    let mut next:HashMap<(i32,i32,i32,i32),bool> = HashMap::new();
   
    for (y,l) in data.iter().enumerate() {
    for (x,c) in l.chars().enumerate()
    {            
        if c=='#' { next.insert((x as i32,y as i32,0,0),true); }        
    }}

    for _ in 0..6
    {
        simulate4d(&next.clone(),&mut next);    
        next = next.into_iter().filter(|x| x.1).collect();
    }

    next.len() as i32 
     */
    0
}




pub fn part1(data:&[String])->i32
{
    solve1(&data)
}

pub fn part2(data:&[String])->i32
{
0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day22");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "on x=-20..26,y=-36..17,z=-47..7".to_string(),
        "on x=-20..33,y=-21..23,z=-26..28".to_string(),
        "on x=-22..28,y=-29..23,z=-38..16".to_string(),
        "on x=-46..7,y=-6..46,z=-50..-1".to_string(),
        "on x=-49..1,y=-3..46,z=-24..28".to_string(),
        "on x=2..47,y=-22..22,z=-23..27".to_string(),
        "on x=-27..23,y=-28..26,z=-21..29".to_string(),
        "on x=-39..5,y=-6..47,z=-3..44".to_string(),
        "on x=-30..21,y=-8..43,z=-13..34".to_string(),
        "on x=-22..26,y=-27..20,z=-29..19".to_string(),
        "off x=-48..-32,y=26..41,z=-47..-37".to_string(),
        "on x=-12..35,y=6..50,z=-50..-2".to_string(),
        "off x=-48..-32,y=-32..-16,z=-15..-5".to_string(),
        "on x=-18..26,y=-33..15,z=-7..46".to_string(),
        "off x=-40..-22,y=-38..-28,z=23..41".to_string(),
        "on x=-16..35,y=-41..10,z=-47..6".to_string(),
        "off x=-32..-23,y=11..30,z=-14..3".to_string(),
        "on x=-49..-5,y=-3..45,z=-29..18".to_string(),
        "off x=18..30,y=-20..-8,z=-3..13".to_string(),
        "on x=-41..9,y=-7..43,z=-33..15".to_string(),
        "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877".to_string(),
        "on x=967..23432,y=45373..81175,z=27513..53682".to_string(),
    ];
    assert_eq!(part1(&v),590784);
}

#[test]
fn test2()
{
    let v = vec![
        "on x=-5..47,y=-31..22,z=-19..33".to_string(),
        "on x=-44..5,y=-27..21,z=-14..35".to_string(),
        "on x=-49..-1,y=-11..42,z=-10..38".to_string(),
        "on x=-20..34,y=-40..6,z=-44..1".to_string(),
        "off x=26..39,y=40..50,z=-2..11".to_string(),
        "on x=-41..5,y=-41..6,z=-36..8".to_string(),
        "off x=-43..-33,y=-45..-28,z=7..25".to_string(),
        "on x=-33..15,y=-32..19,z=-34..11".to_string(),
        "off x=35..47,y=-46..-34,z=-11..5".to_string(),
        "on x=-14..36,y=-6..44,z=-16..29".to_string(),
        "on x=-57795..-6158,y=29564..72030,z=20435..90618".to_string(),
        "on x=36731..105352,y=-21140..28532,z=16094..90401".to_string(),
        "on x=30999..107136,y=-53464..15513,z=8553..71215".to_string(),
        "on x=13528..83982,y=-99403..-27377,z=-24141..23996".to_string(),
        "on x=-72682..-12347,y=18159..111354,z=7391..80950".to_string(),
        "on x=-1060..80757,y=-65301..-20884,z=-103788..-16709".to_string(),
        "on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856".to_string(),
        "on x=-52752..22273,y=-49450..9096,z=54442..119054".to_string(),
        "on x=-29982..40483,y=-108474..-28371,z=-24328..38471".to_string(),
        "on x=-4958..62750,y=40422..118853,z=-7672..65583".to_string(),
        "on x=55694..108686,y=-43367..46958,z=-26781..48729".to_string(),
        "on x=-98497..-18186,y=-63569..3412,z=1232..88485".to_string(),
        "on x=-726..56291,y=-62629..13224,z=18033..85226".to_string(),
        "on x=-110886..-34664,y=-81338..-8658,z=8914..63723".to_string(),
        "on x=-55829..24974,y=-16897..54165,z=-121762..-28058".to_string(),
        "on x=-65152..-11147,y=22489..91432,z=-58782..1780".to_string(),
        "on x=-120100..-32970,y=-46592..27473,z=-11695..61039".to_string(),
        "on x=-18631..37533,y=-124565..-50804,z=-35667..28308".to_string(),
        "on x=-57817..18248,y=49321..117703,z=5745..55881".to_string(),
        "on x=14781..98692,y=-1341..70827,z=15753..70151".to_string(),
        "on x=-34419..55919,y=-19626..40991,z=39015..114138".to_string(),
        "on x=-60785..11593,y=-56135..2999,z=-95368..-26915".to_string(),
        "on x=-32178..58085,y=17647..101866,z=-91405..-8878".to_string(),
        "on x=-53655..12091,y=50097..105568,z=-75335..-4862".to_string(),
        "on x=-111166..-40997,y=-71714..2688,z=5609..50954".to_string(),
        "on x=-16602..70118,y=-98693..-44401,z=5197..76897".to_string(),
        "on x=16383..101554,y=4615..83635,z=-44907..18747".to_string(),
        "off x=-95822..-15171,y=-19987..48940,z=10804..104439".to_string(),
        "on x=-89813..-14614,y=16069..88491,z=-3297..45228".to_string(),
        "on x=41075..99376,y=-20427..49978,z=-52012..13762".to_string(),
        "on x=-21330..50085,y=-17944..62733,z=-112280..-30197".to_string(),
        "on x=-16478..35915,y=36008..118594,z=-7885..47086".to_string(),
        "off x=-98156..-27851,y=-49952..43171,z=-99005..-8456".to_string(),
        "off x=2032..69770,y=-71013..4824,z=7471..94418".to_string(),
        "on x=43670..120875,y=-42068..12382,z=-24787..38892".to_string(),
        "off x=37514..111226,y=-45862..25743,z=-16714..54663".to_string(),
        "off x=25699..97951,y=-30668..59918,z=-15349..69697".to_string(),
        "off x=-44271..17935,y=-9516..60759,z=49131..112598".to_string(),
        "on x=-61695..-5813,y=40978..94975,z=8655..80240".to_string(),
        "off x=-101086..-9439,y=-7088..67543,z=33935..83858".to_string(),
        "off x=18020..114017,y=-48931..32606,z=21474..89843".to_string(),
        "off x=-77139..10506,y=-89994..-18797,z=-80..59318".to_string(),
        "off x=8476..79288,y=-75520..11602,z=-96624..-24783".to_string(),
        "on x=-47488..-1262,y=24338..100707,z=16292..72967".to_string(),
        "off x=-84341..13987,y=2429..92914,z=-90671..-1318".to_string(),
        "off x=-37810..49457,y=-71013..-7894,z=-105357..-13188".to_string(),
        "off x=-27365..46395,y=31009..98017,z=15428..76570".to_string(),
        "off x=-70369..-16548,y=22648..78696,z=-1892..86821".to_string(),
        "on x=-53470..21291,y=-120233..-33476,z=-44150..38147".to_string(),
        "off x=-93533..-4276,y=-16170..68771,z=-104985..-24507".to_string(),
            ];
    assert_eq!(part2(&v),2758514936282235);
}