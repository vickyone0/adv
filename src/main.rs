fn main(){
    let mut movies = ["Inception", "The Matrix", "Interstellar"];

    let  slice_movies = &mut movies[1..3];

    for movie in slice_movies.iter_mut() {
        println!("{}", movie);
        *movie = "Updated Movie";
    }

}