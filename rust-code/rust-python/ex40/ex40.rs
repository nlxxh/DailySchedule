fn main(){
    let happy_bday = Song::init(vec!["Happy birthday to you".into(), "I don't want to get sued".into(),"So I'll stop right there".into()]);
    let bulls_on_parade = Song::init(vec!["They rally around tha family".into(),"With pockets full of shells".into()]);
    happy_bday.sing_me_a_song();
    bulls_on_parade.sing_me_a_song();
    let song1 =  Song::init(vec!["Happy birthday to you".into(),"I don't want to get sued".into(),"So I'll stop right there".into()]);
    let song2 =  Song::init(vec!["They rally around the family".into(),"With pockets full of shells".into()]);
    let song3 =  Song::init(vec!["Never mind I find".into(),"Some one like you".into()]);
    let disk = vec![song1, song2, song3];
    let mut mycd = Disk::init(disk);
    let s1=mycd.jump();
    s1.sing_me_a_song();
    let s2=&mut mycd.next();
    s2.sing_me_a_song(); 
    let s3=&mut mycd.next();
    s3.sing_me_a_song();
    let s1=&mut mycd.prev();
    s1.sing_me_a_song();
    let s2=&mut mycd.prev();
    s2.sing_me_a_song(); 
    
}
#[derive(Clone)]
struct Song{
    lyrics:Vec<String>,
}
struct Disk{
    disk:Vec<Song>,
    index:u32,
}
impl Song{
    fn init(lyrics:Vec<String>)->Self{
        Song{lyrics}
    }
    fn sing_me_a_song(&self){
        for line in &self.lyrics{
            println!("{}",line);
        }
    }
}
impl Disk{
    fn init(disk:Vec<Song>)->Self{
        Disk{index:0,disk}
    }
    fn jump(&self)-> Song{
        self.disk[self.index as usize].clone()
    }
    fn next(& mut self)-> Song{
        self.index = (self.index + 1) % self.disk.len() as u32;
        self.jump()
    }
    fn prev(& mut self)->Song{
        self.index = (self.index - 1) % self.disk.len() as u32;
        self.jump()
    }
}