// Strict unbreakable prompt for the White & Red Bow Christmas Napkins
// Copy everything below this line and paste into Google Flow

fn main() {
    let command = "Generate exactly one highly photorealistic real-photo lifestyle product advertisement, square 1000×1000 pixels, PNG, high resolution, ultra sharp details, suitable for scaling.";

    let product = "
    PRODUCT (use ONLY this exact item from the reference images):
    - White paper Christmas napkins
    - Large red ribbon and bow design that looks like a wrapped gift
    - Accurate real-life size and proportions
    - Exact white colour, red bow details and texture from the reference images
    ";

    let rules = "
    The napkins must look like a real photograph, fully visible, extremely sharp, and the absolute main focus. Show a neat stack or a few unfolded so the design is clear. Correct real-world scale is mandatory. No packaging.

    SCENE:
    - Simple elegant home interior with soft natural daylight
    - Napkins placed on a wooden dining table or festive surface at their true real size
    - Clean minimal background
    - Maximum 2–3 subtle festive accents only

    STRICT FORBIDDEN:
    - Extra text, logos, price tags, Shoprite branding, crowds, box shots, full faces, heavy snow, clutter
    - Any unrealistic size or artificial look
    ";

    let final_order = "Final order: The exact white and red bow Christmas napkins must look like a real photograph, correct real-life size, highly photorealistic, high resolution, clearly visible and dominant. Zero deviation.";

    println!("{}", command);
    println!("{}", product);
    println!("{}", rules);
    println!("{}", final_order);
}
