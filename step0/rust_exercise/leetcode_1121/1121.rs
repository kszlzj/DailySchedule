impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut st = Vec::new();
        let mut count:i32=0;
        for i in s.chars() {
            if (st.len()==0||st[st.len()-1]==i){
                st.push(i);
            }else{
                st.pop();
            }
            if(st.len()==0){
                count+=1;
            }
        }
        count
    }
}