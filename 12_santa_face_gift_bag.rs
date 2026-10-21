// Strict unbreakable prompt for the Santa Face Christmas Gift Bag
// Copy everything below this line and paste into Google Flow

fn main() {
    let command = "Generate exactly one highly photorealistic real-photo lifestyle product advertisement, square 1000×1000 pixels, PNG, high resolution, ultra sharp details, suitable for scaling.";

    let product = "
    PRODUCT (use ONLY this exact item from the reference images):
    - Light blue / teal Christmas gift bag
    - Large cute Santa face with red hat, white beard and red nose
    - “Merry Christmas” text
    - Red rope handles
    - Accurate real-life size and proportions
    - Exact colours, illustration details and bag shape from the reference images
    ";

    let rules = "
    The gift bag must look like a real photograph, fully visible, extremely sharp, and the absolute main focus. Correct real-world scale is mandatory. No packaging.

    SCENE:
    - Simple elegant home interior with soft natural daylight
    - Gift bag standing on a wooden table or shelf at its true real size
    - Clean minimal background
    - Maximum 2–3 subtle festive accents only

    STRICT FORBIDDEN:
    - Extra text, logos, price tags, Shoprite branding, crowds, box shots, full faces, heavy snow, clutter
    - Any unrealistic size or artificial look
    ";

    let final_order = "Final order: The exact Santa face Christmas gift bag must look like a real photograph, correct real-life size, highly photorealistic, high resolution, clearly visible and dominant. Zero deviation.";

    println!("{}", command);
    println!("{}", product);
    println!("{}", rules);
    println!("{}", final_order);
}
