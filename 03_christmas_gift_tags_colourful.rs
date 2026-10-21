// Strict unbreakable prompt for the Christmas Gift Tags
// Copy everything below this line and paste into Google Flow

fn main() {
    let command = "Generate exactly one highly photorealistic real-photo lifestyle product advertisement, square 1000×1000 pixels, PNG, high resolution, ultra sharp details, suitable for scaling.";

    let product = "
    PRODUCT (use ONLY this exact item from the reference images):
    - Set of colourful Christmas gift tags
    - Designs with Christmas trees, Santa, bells, “Merry Christmas” and “Jolly time”
    - Accurate real-life size and proportions
    - Exact colours, illustrations and tag shapes from the reference images
    ";

    let rules = "
    The gift tags must look like a real photograph, fully visible, extremely sharp, and the absolute main focus. Show several tags clearly so the designs are readable. Correct real-world scale is mandatory. No packaging.

    SCENE:
    - Simple elegant home interior with soft natural daylight
    - Gift tags arranged neatly on a wooden table or attached to a small gift
    - Clean minimal background
    - Maximum 2–3 subtle festive accents only

    STRICT FORBIDDEN:
    - Extra text, logos, price tags, Shoprite branding, crowds, box shots, full faces, heavy snow, clutter
    - Any unrealistic size or artificial look
    ";

    let final_order = "Final order: The exact Christmas gift tags must look like a real photograph, correct real-life size, highly photorealistic, high resolution, clearly visible and dominant. Zero deviation.";

    println!("{}", command);
    println!("{}", product);
    println!("{}", rules);
    println!("{}", final_order);
}
