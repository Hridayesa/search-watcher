use std::collections::HashMap;
use url_escape;
use strfmt::strfmt;
pub trait SearchUrl{
    fn get_url(&self, params: &HashMap<String,String>)->String;
}

// pub struct SearchParams{
//     search_str: String,
// }

pub struct IceTradeUrl{
    template: String,
}

impl SearchUrl for IceTradeUrl{
    fn get_url(&self, params: &HashMap<String,String>) -> String {
        let res = strfmt(self.template.as_str(), params).unwrap();
        url_escape::encode_fragment(res.as_str()).to_string()
    }
}

#[cfg(test)]
mod tests {
    // use std::fmt::Debug;
    use super::*;
    use map_macro::hash_map;

    #[test]
    fn request_1(){
        let client = reqwest::blocking::Client::new();
        let body = client.get("https://icetrade.by")
            .header("User-Agent", "PostmanRuntime/7.32.3")
            .send();

        // let body = reqwest::blocking::get("https://icetrade.by");
        // let body = reqwest::blocking::get(text_url);
        println!("{:#?}", body);
        println!("<<<<<{:?}>>>>>>", body.unwrap().text().unwrap());
    }
    #[test]
    fn request(){
        let url = IceTradeUrl{
            template: "https://icetrade.by/search/auctions?search_text={search_text}&search=%D0%9D%D0%B0%D0%B9%D1%82%D0%B8&zakup_type%5B1%5D=1&zakup_type%5B2%5D=1&auc_num=&okrb=&company_title=&establishment=0&industries=&period=&created_from=&created_to=&request_end_from=&request_end_to=&t%5BTrade%5D=1&t%5BeTrade%5D=1&t%5BsocialOrder%5D=1&t%5BsingleSource%5D=1&t%5BAuction%5D=1&t%5BRequest%5D=1&t%5BcontractingTrades%5D=1&t%5Bnegotiations%5D=1&t%5BOther%5D=1&r%5B1%5D=1&r%5B2%5D=2&r%5B7%5D=7&r%5B3%5D=3&r%5B4%5D=4&r%5B6%5D=6&r%5B5%5D=5&sort=num%3Adesc&sbm=1&onPage=20".to_string(),
        };
        let params = hash_map! {
            String::from("search_text") => String::from("Server"),
        };
        let text_url = url.get_url(&params);
        // let body = reqwest::blocking::get("https://www.rust-lang.org");
        
        let client = reqwest::blocking::Client::new();
        let body = client.get(text_url)
            .header("User-Agent", "PostmanRuntime/7.32.3")
            .send();

        // let body = reqwest::blocking::get(text_url);
        println!("{:#?}", body);
        println!("<<<<<{}>>>>>>", body.unwrap().text().unwrap());
    }
    #[test]
    fn ice_trade_url(){
        let url = IceTradeUrl{
            template: String::from("https://icetrade.by/search/auctions?search_text={search_text}&search=%D0%9D%D0%B0%D0%B9%D1%82%D0%B8&zakup_type%5B1%5D=1&zakup_type%5B2%5D=1&auc_num=&okrb=&company_title=&establishment=0&industries=&period=&created_from=&created_to=&request_end_from=&request_end_to=&t%5BTrade%5D=1&t%5BeTrade%5D=1&t%5BsocialOrder%5D=1&t%5BsingleSource%5D=1&t%5BAuction%5D=1&t%5BRequest%5D=1&t%5BcontractingTrades%5D=1&t%5Bnegotiations%5D=1&t%5BOther%5D=1&r%5B1%5D=1&r%5B2%5D=2&r%5B7%5D=7&r%5B3%5D=3&r%5B4%5D=4&r%5B6%5D=6&r%5B5%5D=5&sort=num%3Adesc&sbm=1&onPage=20"),
        };
        let params = hash_map! {
            String::from("search_text") => String::from("Server, Сервер"),
        };
        let res = url.get_url(&params);
        println!("{}", res);
        // assert_eq!(res, )
    }
    #[test]
    fn it_works() {
        let result = 2*2;
        println!("{}", url_escape::encode_fragment("a > b?"));
        assert_eq!(result, 4);
    }

    #[test]
    fn fmt(){
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "bob");
        vars.insert("job".to_string(), "python developer");

        let fmt = "hi, my name is {name} and I am a {job}!".to_string();
        assert_eq!(
            strfmt(&fmt, &vars).unwrap(),
            "hi, my name is bob and I am a python developer!")
    }

    #[test]
    fn fmt_escape(){
        let mut vars = HashMap::new();
        vars.insert("search_text".to_string(), "Server");
        let fmt = String::from("https://icetrade.by/search/auctions?search_text={search_text}&search=%D0%9D%D0%B0%D0%B9%D1%82%D0%B8&zakup_type%5B1%5D=1&zakup_type%5B2%5D=1&auc_num=&okrb=&company_title=&establishment=0&industries=&period=&created_from=&created_to=&request_end_from=&request_end_to=&t%5BTrade%5D=1&t%5BeTrade%5D=1&t%5BsocialOrder%5D=1&t%5BsingleSource%5D=1&t%5BAuction%5D=1&t%5BRequest%5D=1&t%5BcontractingTrades%5D=1&t%5Bnegotiations%5D=1&t%5BOther%5D=1&r%5B1%5D=1&r%5B2%5D=2&r%5B7%5D=7&r%5B3%5D=3&r%5B4%5D=4&r%5B6%5D=6&r%5B5%5D=5&sort=num%3Adesc&sbm=1&onPage=20");
        let search_string: String = String::from("https://icetrade.by/search/auctions?search_text=Server&search=%D0%9D%D0%B0%D0%B9%D1%82%D0%B8&zakup_type%5B1%5D=1&zakup_type%5B2%5D=1&auc_num=&okrb=&company_title=&establishment=0&industries=&period=&created_from=&created_to=&request_end_from=&request_end_to=&t%5BTrade%5D=1&t%5BeTrade%5D=1&t%5BsocialOrder%5D=1&t%5BsingleSource%5D=1&t%5BAuction%5D=1&t%5BRequest%5D=1&t%5BcontractingTrades%5D=1&t%5Bnegotiations%5D=1&t%5BOther%5D=1&r%5B1%5D=1&r%5B2%5D=2&r%5B7%5D=7&r%5B3%5D=3&r%5B4%5D=4&r%5B6%5D=6&r%5B5%5D=5&sort=num%3Adesc&sbm=1&onPage=20");
        let url = strfmt(&fmt, &vars).unwrap();
        println!("{}",url);
        assert_eq!(search_string, url);
        assert_eq!(url_escape::encode_fragment(&url), url)
    }

    #[test]
    fn cord(){
        println!("{}", 2*20 + 2*15 + 5*7 + 3*5 + 3*4 + 4*3);
    }
}