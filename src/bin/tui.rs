use cursive::{
    self,
    views::{
        TextView
    }
};

fn main() {
	let mut siv = cursive::default();

	siv.add_global_callback(
        'q',
        |s| s.quit()
    );

    siv.add_layer(
        TextView::new(
            "Turn's Cock Tier Evaluator\n       <q> to quit."
        )
    );
    
    siv.run()
}