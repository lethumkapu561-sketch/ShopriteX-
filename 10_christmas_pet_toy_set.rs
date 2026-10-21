// Strict unbreakable prompt for the Christmas Pet Toy Set (Bear & Dog)
// Copy everything below this line and paste into Google Flow

fn main() {
    let command = "Generate exactly one highly photorealistic real-photo lifestyle product advertisement, square 1000×1000 pixels, PNG, high resolution, ultra sharp details, suitable for scaling.";

    let product = "
    PRODUCT (use ONLY this exact item from the reference images):
    - Two plush Christmas pet toys: a brown teddy bear and a black-and-white dog
    - Both wearing red Santa hats and festive scarves
    - Hanging loops attached
    - Accurate real-life size and proportions
    - Exact colours, plush texture and details from the reference images
    ";

    let rules = "
    The pet toys must look like a real photograph, fully visible, extremely sharp, and the absolute main focus. Show both toys clearly. Correct real-world scale is mandatory. No packaging.

    SCENE:
    - Simple elegant home interior with soft natural daylight
    - Pet toys placed or hanging naturally so usage is clear
    - Clean minimal background
    - Maximum 2–3 subtle festive accents only

    STRICT FORBIDDEN:
    - Extra text, logos, price tags, Shoprite branding, crowds, box shots, full faces, heavy snow, clutter
    - Any unrealistic size or artificial look
    ";

    let final_order = "Final order: The exact Christmas pet toy set (bear and dog) must look like a real photograph, correct real-life size, highly photorealistic, high resolution, clearly visible and dominant. Zero deviation.";

    println!("{}", command);
    println!("{}", product);
    println!("{}", rules);
    println!("{}", final_order);
}
