impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        let l=a.len();
        if l==0{
            return false;
        }
        let mut index=0;
        while (index<l-1 && a[index]<a[index+1]) {
            index+=1;
        }
        if index == 0 || index + 1 == l {
            return false
        }
        while (index<l-1&&a[index]>a[index+1]){
            index+=1;
        }
        index+1==l
    }
}