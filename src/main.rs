fn main(){
    let  movies = ["Inception", "The Matrix", "Interstellar"];

    let slice_movies = &movies[1..3];

    for movie in slice_movies {
        println!("{}", movie);
    }

}