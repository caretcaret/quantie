// Let's begin with a review of ordinary, classical state.
struct OrdinaryVariable<T: Clone> {
  // A variable stores a value and keeps it over time.
  value: T,
}
// Okay...

impl<T: Clone> OrdinaryVariable<T> {
  // You can allocate a new variable to hold things...
  fn new(initial_value: T) -> OrdinaryVariable<T> {
    OrdinaryVariable { value: initial_value }
  }
  // You can change the value of the variable...
  fn set(&mut self, other: &mut OrdinaryVariable<T>) {
    self.value = other.get()
  }
  // And you can read the variable.
  fn get(&mut self) -> T {
    self.value.clone()
  }
  // So far so good, right?
}

// For example...
fn example_1() {
  let mut bit = OrdinaryVariable::new(false);
  println!("First it's {}", bit.get());
  let mut true_bit = OrdinaryVariable::new(true);
  bit.set(&mut true_bit);
  println!("Now it's {}", bit.get());

  let mut integer = OrdinaryVariable::new(0);
  println!("First it's {}", integer.get());
  let mut forty_two = OrdinaryVariable::new(42);
  integer.set(&mut forty_two);
  println!("Now it's {}", integer.get());

  let mut real = OrdinaryVariable::new(0.0);
  println!("First it's {}", real.get());
  let mut pi = OrdinaryVariable::new(3.14159);
  real.set(&mut pi);
  println!("Now it's {}", real.get());

  let mut text = OrdinaryVariable::new("");
  println!("First it's \"{}\"", text.get());
  let mut hello_world = OrdinaryVariable::new("Hello, world!");
  text.set(&mut hello_world);
  println!("Now it's \"{}\"", text.get());
}

// You can also do computations! Here is an example of an operation on one state:
fn negate(var: &mut OrdinaryVariable<bool>) {
  let mut result = OrdinaryVariable::new(!var.get());
  var.set(&mut result);
}

// And one on two, with another to hold the output...
fn and(output: &mut OrdinaryVariable<bool>, left: &mut OrdinaryVariable<bool>,
       right: &mut OrdinaryVariable<bool>) {
  let mut result = OrdinaryVariable::new(left.get() && right.get());
  output.set(&mut result);
}

// Now you can do whatever your heart desires.
fn example_2() {
  let mut x = OrdinaryVariable::new(true);
  let mut y = OrdinaryVariable::new(true);
  let mut output = OrdinaryVariable::new(false);
  negate(&mut y);
  and(&mut output, &mut x, &mut y);
  negate(&mut output);
  println!("x => y == {}", output.get());
}
// O....kay! That was pretty trite. Trust me it'll help other things make sense.
// Um, that's pretty much all I had to say on classical state. Moving on!

// Now let's welcome ourselves to the world of probabilistic state, or random variables.
// So you can do what with a random variable now?
// Well, actually, everything that OrdinaryVariables can do. Ya make a new one, ya set it, ya look at it.
// Watch, it's easy.
extern crate rand;
use rand::Rng;
fn example_3() {
  let mut rng = rand::thread_rng();
  let mut coin = OrdinaryVariable::new(rng.gen_weighted_bool(3));  // 1 in 3 chance? not fair :<
  println!("coin is {}", if coin.get() { "heads" } else { "tails" });
  let mut dice = OrdinaryVariable::new(rng.gen_range(0, 6));
  println!("dice is {}", dice.get() + 1);
  let mut weather = OrdinaryVariable::new(rng.gen_range(11.0, 14.5));
  println!("weather is {} degrees", weather.get());
}
// So, that's randomness. Now, on to quantum--

// WAIT WHAT'S THAT RNG THINGY. I feel cheated :'(

// Ya got me... I hid all the randomness inside the random number generator!
// Basically, randomness is the idea that there are lots of possible events that can happen,
// and each event has a weighting which is the probability. And all of the probabilities add to 1. Delicious.
// But in reality, only one of the events will ever occur! When you experience the randomness,
// something, someone (RNGesus) chose the event that you'll observe.
// If you do the same random thing lots of times, you'll see that the proportion of times you get some event
// is close to the probability of that event. The more you perform it, the closer it gets.
// So actually, what is happening above is that you're being forced to experience the randomness when the
// variable is generated. But what does it look like if we delay the randomness to happen when something
// observes it?

// Let's stick with the coin case (no pun intended):
struct RandomBool {
  // Coin has some probability of being heads (true).
  prob_true: f64,
  // We don't need to write a prob_false here because there are only two possible values,
  // so the other probability is 1 - prob_true.
}
impl RandomBool {
  // You can allocate an unfair (or fair, if you wish) coin to flip.
  fn new(prob_true: f64) -> RandomBool {
    RandomBool { prob_true: prob_true }
  }
  // You can set its value to another random bool.
  fn set(&mut self, other: &mut RandomBool) {
    self.prob_true = if other.get() { 1.0 } else { 0.0 };
  }
  // And now you can flip it!
  fn get(&mut self) -> bool {
    let mut rng = rand::thread_rng();
    let result = rng.next_f64() <= self.prob_true;
    self.prob_true = if result { 1.0 } else { 0.0 };
    result
  }
}

// Just so you know, if you copy a random variable into another random variable, they'll have the same
// value, always. If the second coin was random relative to the first coin, that wouldn't be a very
// effective copy, now would it?
fn example_4() {
  // Skip the penny, it's a dead currency.
  let mut nickel = RandomBool::new(0.5);
  let mut dime = RandomBool::new(0.0);
  dime.set(&mut nickel);
  let mut quarter = if dime.get() { RandomBool::new(0.8) } else { RandomBool::new(0.4) };
  println!("{} and {}. See, you get nickeled and dimed all the same!", nickel.get(), dime.get());
  println!("And the quarter is {}: likely same as dime, but not always.", quarter.get());
  // ...And slightly more favorable to be heads.
}

// So that's one picture of randomness, where the randomness happens when you observe it.
// And we started with the picture that randomness occurs when you create the variable, so everything
// ends up being an ordinary variable.
// There's still a third way of looking at randomness. What if we delayed observing the randomness...
// forever? There wouldn't be an observer running any experiments. Instead what we end up doing is
// inspecting the probabilities themselves, essentially playing goddess by running all experiments
// at once.

// *boom*
// Oh my God, Rick, what the hell just happened?
// Don't worry, Morty, uhh... We needed... we just needed, uhh... we needed some more modeling power.
// *burp* I'll explain everything.

// TO BE CONTINUED...

fn main() {
  example_1();
  example_2();
  example_3();
  example_4();
}
