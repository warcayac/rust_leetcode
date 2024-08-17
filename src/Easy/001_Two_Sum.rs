struct Solution;

impl Solution {
  fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for (i, x) in nums.iter().enumerate() {
      // esta es una asignación condicional, si el método "get" retorna un valor (del subtipo Some para Option), 
      // entonces se asigna a la variable "j" el valor retornado que corresponde a la clave de la entrada,
      // seguidamente ejecuta ejecuta el bloque de código asociado. Si el valor retornado es del subtipo None,
      // entonces no se ejecuta el bloque de código asociado.
      if let Some(&j) = map.get(x) {
        // termina el bucle y sale de la función retornando un vector
        return vec![j as i32, i as i32];
      }
      map.insert(target - x, i);
    }
    // si no hay solución, retorna un vector vacío
    vec![]
  }
}

struct Solution2;
impl Solution2 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
        // println!("{:?}", nums);
        // nums es ahora un vector de tuplas, donde el primer valor de la tupla es el índice (0,...)
        // del actual elemento del vector, y el segundo valor es el elemento del vector en curso.
        
        // iterar sobre nums
        while let Some(next) = nums.pop() {
            let (this_index, this_value) = next;
            
            for (other_index, other_value) in &nums {
                if other_value + this_value == target {
                    return vec![this_index as i32, *other_index as i32]
                }
            }
        }
        
        vec![]
    }
}

fn main() {
  // let _solution = Solution::two_sum(vec![2, 7, 11, 15], 9);
  let _solution = Solution2::two_sum(vec![2,6,4,9,8,16], 14);
  println!("{:?}", _solution);
  println!("Job done!");
}