// vecs2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.



fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {//.iter_mut() this will returns a mutable iterator to
                           // loop through all the elements of the collection

        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *i *= 2;//The question is why i used "*". For this knowledge first you have to learn
                //how ownership works. For who knows already, .iter_mut() loops all units in the vector 
                //with &mut i32. Which means a mutable i32 reference. If you want to reach a value at the
                //given address, you can use "*".
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|num| {//|num| is a closure you will learn this later. But for now you 
                        //can think like vec_loop function. For who knows, .iter() loops the values,
                        //.map() collects values under a roof and closure num takes values 
                        //and use them with num * 2

        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        num * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
