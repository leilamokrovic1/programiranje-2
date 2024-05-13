// Napišite različne funkcije, ki kot argument sprejmejo zaprtje (closure) in ga pokličejo z različnimi argumenti, ki ustrezajo sledečim ocaml tipom:
// Tip zaprtja naj bo čim bolj splošen (Fn, FnOnce, FnMut).
//  (int -> int) -> int -> int
//  ('a -> 'b) -> 'a -> 'b
//  ('a -> 'a -> 'b) -> 'a -> 'a -> 'b
//  map: ('a -> 'b) -> 'a list -> 'b list  (Uporabite Vec<T> namesto list, predpostavite, da funkcija ne spremeni elementov seznama)
//  map_and_mutate: ('a -> 'b) -> 'a list -> 'b list // Definirajte funkcijo tako, da lahko zaprtje spremeni elemente seznama
//  ponavljaj: int -> ('a -> 'a) -> 'a -> 'a // Ponovi zaprtje n-krat
//  filter: ('a -> bool) -> 'a list -> 'a list // Vrne seznam elementov, ki zadoščajo pogoju - uporabite Vec<T> namesto list in že vgrajeno funkcijo filter

// Vzemite zaporedja iz prejšnjih vaj in naredite nov objekt, ki sprejme zaporedje in ga naredi iterabilnega
//4. VAJE

// fn apply <F: FnOnce(i32) -> i32>(func: F, a: i32) -> i32 {
//     func(a)
// }

// fn apply_b <T1, T2, F: FnOnce(T1) -> T2>(func: F, a: T1) -> T2 {
//     func(a)
// }

// fn main() {
//    // let k_dva = |x| {x * 2};
//     let neki = 12;
//     println!("apply {}",apply(|x| x * neki, 10));
//     println!("apply_b {}",apply_b(|x| x * neki, 10))