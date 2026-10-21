// Strict unbreakable prompt for the Christmas Carousel Snow Globe
// Copy everything below this line and paste into Google Flow

fn main() {
    let command = "Generate exactly one highly photorealistic real-photo lifestyle product advertisement, square 1000×1000 pixels, PNG, high resolution, ultra sharp details, suitable for scaling.";

    let product = "
    PRODUCT (use ONLY this exact item from the reference images):
    - Christmas snow globe with red base
    - Detailed carousel scene inside with reindeer, snowmen, Christmas trees and festive figures
    - Soft warm internal glow when lit
    - Accurate real-life size and proportions
    - Exact colours, miniature details, glass and base texture from the reference images
    ";

    let rules = "
    The snow globe must look like a real photograph, fully visible, extremely sharp, and the absolute main focus. Correct real-world scale is mandatory. No packaging.

    SCENE:
    - Simple elegant home interior with soft natural daylight
    - Snow globe standing on a wooden table or shelf at its true real size
    - Clean minimal background that highlights the glowing scene inside
    - Maximum 2–3 subtle festive accents only

    STRICT FORBIDDEN:
    - Extra text, logos, price tags, Shoprite branding, crowds, box shots, full faces, heavy snow, clutter
    - Any unrealistic size or artificial look
    ";

    let final_order = "Final order: The exact Christmas carousel snow globe must look like a real photograph, correct real-life size, highly photorealistic, high resolution, clearly visible and dominant. Zero deviation.";

    println!("{}", command);
    println!("{}", product);
    println!("{}", rules);
    println!("{}", final_order);
}
