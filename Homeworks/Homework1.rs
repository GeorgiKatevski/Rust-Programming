pub fn fizzbuzz(n: usize) -> Vec<String> {
    
    let mut vec: Vec<String> = Vec::with_capacity(n);
        if n==0
        {
            return vec;
        }

for i in 1..n+1
{
    
    if i%3==0 && i%5!=0
    {
        let  s=String::from("Fizz");
        vec.push(s);
    }
    else if i%5==0 && i%3!=0
    {
        vec.push("Buzz".to_string());
    } 
    else if i%5==0 && i%3==0
    {
        vec.push("FizzBuzz".to_string());
    }
    else
    {
    vec.push(i.to_string());
    }


}



return vec;

}

pub fn custom_buzz(n: usize, k1: u8, k2: u8) -> Vec<String> {
    
if k1==0 || k2==0 || k2==1 || k1==1
{
    panic!("Problem!");
        
}

    let mut vec: Vec<String> = Vec::with_capacity(n);
    if n==0
    {
        return vec;
    }

            for i in 1..n+1
            {
            
                if i%usize::from(k1)==0 && i%usize::from(k2)!=0
                    {
                        let  s=String::from("Fizz");
                        vec.push(s);
                    }
                else if i%usize::from(k2)==0 && i%usize::from(k1)!=0
                    {
                        vec.push("Buzz".to_string());
                    }    
                else if i%usize::from(k2)==0 && i%usize::from(k1)==0
                    {
                        vec.push("FizzBuzz".to_string());
                    }
                else
                    {
                        vec.push(i.to_string());
                    }

            }



return vec;
}


/// Параметри:
/// - полета `k1` и `k2`, които са двата делителя, които ще използваме за заместване.
/// - поле `labels`, които са трите етикета, които съответстват на коефициентите
///
/// Ако `n` е 0, очакваме празен вектор за резултат.
/// Ако `k1` или `k2` са 0 или 1, очакваме функцията да panic-не с каквото съобщение изберете.
///
pub struct FizzBuzzer {
    pub k1: u8,
    pub k2: u8,
    pub labels: [String; 3],
}

impl FizzBuzzer {
    /// За всяко число от 1 до `n` включително, искаме съответстващия елемент в резултата да е:
    ///
    /// - Първия String от полето `labels` ако числото се дели на k1, но не на k2
    /// - Втория String от полето `labels` ако числото се дели на k2, но не на k1
    /// - Третия String от полето `labels` ако числото се дели и на k1, и на k2
    /// - Числото конвертирано до низ, във всички други случаи
    ///
    pub fn take(&self, n: usize) -> Vec<String> {
        let mut vec: Vec<String> = Vec::with_capacity(n);
       
        if n==0
        {
            return vec;
        }

        if self.k1==0 || self.k2==0 || self.k2==1 || self.k1==1
            {
                 panic!("Problem!");
            }

    for i in 1..n+1
    {
        if i%usize::from(self.k1)==0 && i%usize::from(self.k2)!=0
        {
            
            vec.push(self.labels[0].to_string());
        }
        else if i % usize::from(self.k1)!=0 && i% usize::from(self.k2)==0
        {
            vec.push(self.labels[1].to_string());
        }
        else if i% usize::from(self.k1)==0 && i% usize::from(self.k2)==0
        {
            vec.push(self.labels[2].to_string());
        }
        else
        {
            vec.push(i.to_string());
        }



    }
        
    return vec;
    }

    /// Параметъра `index` указва кой етикет от полето `labels` променяме, от 0 до 2. Ако подадения
    /// `index` е извън тези рамки, очакваме функцията да panic-не.
    ///
    /// Стойността `value` е низа, който ще сложим на този индекс в полето `labels`.
    ///
    pub fn change_label(&mut self, index: usize, value: &String) 
    {
            if  index>2  
            {
                panic!("Out of range!");
            }

            self.labels[index]=value.to_string();

    }
}

#[test]
fn test_basic() {
    let expected = vec![1.to_string(), 2.to_string(), String::from("Fizz")];

    assert_eq!(fizzbuzz(3), expected);
    assert_eq!(custom_buzz(3, 3, 5), expected);

    let mut fizzbuzzer = FizzBuzzer {
        k1: 3,
        k2: 5,
        labels: [
            String::from("Fizz"),
            String::from("Buzz"),
            String::from("Fizzbuzz")
        ],
    };
    assert_eq!(fizzbuzzer.take(3), expected);
    fizzbuzzer.change_label(0, &String::from("Fiz"));
}
#[test]
#[should_panic]
fn test_custom_panic() {
    custom_buzz(2, 1, 1);
    custom_buzz(2, 0, 2);
    custom_buzz(2, 1, 2);
    custom_buzz(2, 3, 1);
    custom_buzz(2, 0, 0);
    custom_buzz(2, 3, 0);
}
