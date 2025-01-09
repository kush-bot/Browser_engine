use std::{collections::HashMap, vec};



enum NodeType{
    Text(String),
    Element(ElementData),
}
 
struct Node{
    children:Vec<Node>,
    node_type:NodeType,
}


struct ElementData{
    tag_name:String,
    attrs:AttrMap,
}

type  AttrMap = HashMap<String,String>;


fn text(data:String)->Node{

    Node{children:Vec::new(),node_type:NodeType::Text(data)}
}

pub fn elem(tag_name:String,attrs:AttrMap,children:Vec<Node>)->Node{
    Node{
        children,
        node_type:NodeType::Element(ElementData{tag_name,attrs})
    }
}

fn main(){

}