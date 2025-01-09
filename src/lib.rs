



pub struct NewsArticle{
    pub headlines:String,
    pub location:String,
    pub author:String,
    pub content:String
}

pub trait Summary{
    fn summerice(&self)->String;
}


impl Summary for NewsArticle{
    fn summerice(&self)->String{
        format!("{}, by {},{}{}",self.headlines,self.author,self.location,self.content)
    }
}
pub struct Tweet{
    pub username:String,
    pub content:String,
    pub retweet:bool,
    pub replay:bool,
}

impl Summary for  Tweet{
    fn summerice(&self)->String {
        format!("{} tweeted {}",self.username,self.content)
    }
}

pub fn notify<T:Summary>(item:&T){
    print!("{}",item.summerice())
}
pub fn return_summerizable(switch:bool)->impl Summary{
    if switch{
        NewsArticle{
            headlines:String::from("this is headline"),
            location:String::from("somewhere near here"),
            author:String::from("someone"),
            content:String::from("this is content")
        }
    }
    else{
        Tweet{
            username:String::from("theTwitter"),
            content:String::from("the content"),
            retweet:false,
            replay:false,
        }
    }
}
