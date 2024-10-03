struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
    /*
    Explicación:
    Ya que cada hace jugadas óptimas, es decir, siempre quita piedras de forma que el contrario al 
    hacer su movida quede en una posición donde no pueda ganar. Bahjo esta lógica, el problema 
    nos solicita determinar si tenemos chance de ganar según el número de piedras que haya.
    Haciendo un análisis, se puede deducir que si el número de piedras es divisible por 4, entonces
    el jugador que juega en su turno no tiene chance de ganar, ya que el otro jugador puede siempre
    dejar al jugador en una posición donde no pueda ganar.
    */
}

fn main() {
    let tests = [
        4, // false
        1, // true
        2, // true
    ];

    for t in tests.into_iter() {
        let result = Solution::can_win_nim(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
