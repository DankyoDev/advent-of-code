use crate::{errors::AocError, models::DayResult, utils::openfiles::open_input};

#[derive(Copy, Clone)]
enum Dir { Up, Right, Down, Left }

impl Dir {
  #[inline] fn turn(self)->Self {
	match self{Self::Up=>Self::Right,Self::Right=>Self::Down,Self::Down=>Self::Left,Self::Left=>Self::Up}
  }
  #[inline] fn delta(self)->(isize,isize){
	match self{Self::Up=>(-1,0),Self::Right=>(0,1),Self::Down=>(1,0),Self::Left=>(0,-1)}
  }
}

pub fn run()->Result<DayResult,AocError>{
  let input=open_input(2024,6)?;
  let mut g:Vec<Vec<u8>>=input.lines().map(|l|l.as_bytes().to_vec()).collect();
  let h=g.len(); let w=g[0].len();
  
  let(mut sr,mut sc,mut sd)=(0,0,Dir::Up);
  'find:for r in 0..h{for c in 0..w{
	sd=match g[r][c]{b'^'=>Dir::Up,b'v'=>Dir::Down,b'<'=>Dir::Left,b'>' =>Dir::Right,_=>continue};
	sr=r;sc=c;break 'find;
  }}
  
  let mut visited=vec![false;h*w];
  visited[sr*w+sc]=true;
  
  let(mut r,mut c,mut d)=(sr as isize,sc as isize,sd);
  loop{
	let(dr,dc)=d.delta(); let nr=r+dr; let nc=c+dc;
	if nr<0||nc<0||nr>=h as isize||nc>=w as isize {break;}
	if g[nr as usize][nc as usize]==b'#'{d=d.turn();continue;}
	r=nr;c=nc;
	visited[r as usize*w+c as usize]=true;
  }
  
  let part1=visited.iter().filter(|x|**x).count();
  
  let mut part2=0usize;
  
  for idx in 0..h*w{
	if !visited[idx] || idx==(sr*w+sc){continue;}
	let rr=idx/w; let cc=idx%w;
	if g[rr][cc]==b'#'{continue;}
	
	g[rr][cc]=b'#';
	
	let mut seen=vec![false;h*w*4];
	let(mut r,mut c,mut d)=(sr as isize,sc as isize,sd);
	
	let looped=loop{
	  let id=(r as usize*w+c as usize)*4+match d{Dir::Up=>0,Dir::Right=>1,Dir::Down=>2,Dir::Left=>3};
	  if seen[id]{break true;}
	  seen[id]=true;
	  
	  let(dr,dc)=d.delta(); let nr=r+dr; let nc=c+dc;
	  if nr<0||nc<0||nr>=h as isize||nc>=w as isize{break false;}
	  if g[nr as usize][nc as usize]==b'#'{d=d.turn();continue;}
	  r=nr;c=nc;
	};
	
	if looped{part2+=1;}
	
	g[rr][cc]=b'.';
  }
  
  Ok(DayResult::new(part1.to_string(),part2.to_string()))
}
