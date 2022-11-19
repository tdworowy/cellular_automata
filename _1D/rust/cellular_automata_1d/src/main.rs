struct RuleSegment <const NEIGHBORHOOD_SIZE: usize> {
    neighborhood: [u32; NEIGHBORHOOD_SIZE],
    cell_type: u32,
}

fn n_nary(mut number: u32, n: u32) -> Vec<u32>  {
    let mut result = Vec::new();
    if number == 0 {
        result.push(0 as u32);
    } else{ 
      while number > 0 {
        let temp= (number/n, number%n);
        number = temp.0;
        result.push(temp.1);
      }  
    }
    result.reverse();
    result
}

#[test]
fn test_n_ary() {
    assert_eq!(n_nary(110, 2),[1,1,0,1,1,1,0]);
    assert_eq!(n_nary(0, 2),[0]);
    assert_eq!(n_nary(10, 3),[1,0,1]);
}

fn wolfram_number_to_bin(wolfram_number: u32, possible_states: u32, colours_count: u32) ->Vec<u32> {
    let mut wolfram_number_n_ary = n_nary(wolfram_number, colours_count);
    let mut wolfram_number_bin =vec![0;possible_states as usize - wolfram_number_n_ary.len()];
    wolfram_number_bin.append(& mut wolfram_number_n_ary);
    wolfram_number_bin.reverse();
    wolfram_number_bin
}


#[test]
fn test_wolfram_number_to_bin() {
    assert_eq!(wolfram_number_to_bin(110, 8, 2),[0, 1, 1, 1, 0, 1, 1, 0]);
}

fn main() {
    println!("Hello, world!");
}
