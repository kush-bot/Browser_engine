struct Parser{
    pos:usize,
    input_str:String
}


impl Parser{
    fn next_char(&self)->char{
        self.input_str[self.pos..].next().char().unwrap();
    }


    fn start_with(&self,s:&str)->bool{
        self.input_str[self.pos..].start_with(s);
    }

    fn expect(&mut self,s:&str){ 
            if self.start_with(s){
                self.pos+=s.len
            }else{
                panic!("Expected  {:?} at byte {} but it was not found ",s,self.pos);
            }
    }

    fn eof(&self)->bool{
        self.pos>=self.input.len()
    }
    
    fn consume_char(&mut self)->char{
        let c = next_char();
        self.pos+=c.len_utf8();
        return c;
    }

    fn consume_while(&mut self,test:impl Fn(char)->bool)->String{
        let mut result =String::new();
        while !self.eof() && test(self.next_char()){
            result.push(self..consume_char());
        }

        return result;
    }

    fn consume_whitespace(&mut self)->_{
        self.consume_while(char::is_whitespace); 
    }

    fn parse_name(&mut self)->String{
        self.consume_while(|c| matches!(c,'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    fn parse_node(&mut self)->dom::Node{
        if self.start_with("<"){
            self.parse_element();
        }else{
            self.parse_text();
        }
    }

    fn parse_text(&mut self)->dom::Node{
        dom::text(self.consume_while(|c| c!='<'))
    }

    fn parse_element(&mut self)->dom::Node{
        self.expect("<");
        let tag_name=self.parse_name();
        let attrs=self.parse_attributes();
        self.expect(">");

        let children = self.parse_nodes();

        self.expect("</");
        self.expect(tag_name);
        self.expect(">");


        return dom::elem(tag_name,attrs,children);


    }

    fn parse_attrs(&mut self)->(String,String){
        let name=self.parse_name();
        self.expect("=");
        let values=self.parse_attr_values();
        return (name,values);
    }

    fn parse_attr_values(&mut self)->String{
        let open_quotes=self.consume_char();
        assert!(open_quotes=='"' || open_quotes=='\'');
        let value = self.consume_while(|c| c!=open_quotes);
        let close_quotes=self.consume_char();
        assert_eq!(open_quotes,close_quotes);
        return value;
    }   

    fn parse_attributes(&mut self)->dom::AttrMap{
        let mut attributes=HashMap::new();
        loop{
            self.consume_whitespace();
            if self.next_char()==">"{
                break;
            }

            let (name,value)=self.parse_attr();
            attributes.insert(name,value);

        }
        return attributes;
    }

    fn parse_nodes(&mut self)->Vec<dom::Node>{
        let mut nodes=Vec::new();
        loop{
            self.consume_whitespace();
            if self.eof() || self.starts_with("</"){
                break
            }
        }
    }
}
