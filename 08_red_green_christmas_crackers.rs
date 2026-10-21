// Strict unbreakable prompt for the Red & Green Christmas Crackers
// Copy everything below this line and paste into Google Flow

fn main() {
    let command = "Generate exactly one highly photorealistic real-photo lifestyle product advertisement, square 1000×1000 pixels, PNG, high resolution, ultra sharp details, suitable for scaling.";

    let product = "
    PRODUCT (use ONLY this exact item from the reference images):
    - Traditional Christmas crackers in red and green
    - Classic twisted ends with festive patterns
    - Accurate real-life size and proportions
    - Exact colours, patterns and texture from the reference images
    ";

    let rules = "
    The crackers must look like a real photograph, fully visible, extremely sharp, and the absolute main focus. Show a neat group so the design is clear. Correct real-world scale is mandatory. No packaging.

    SCENE:
    - Simple elegant home interior with soft natural daylight
    - Christmas crackers arranged on a wooden dining table at their true real size
    - Clean minimal background
    - Maximum 2–3 subtle festive accents only

    STRICT FORBIDDEN:
    - Extra text, logos, price tags, Shoprite branding, crowds, box shots, full faces, heavy snow, clutter
    - Any unrealistic size or artificial look
    ";

    let final_order = "Final order: The exact red and green Christmas crackers must look like a real photograph, correct real-life size, highly photorealistic, high resolution, clearly visible and dominant. Zero deviation.";

    println!("{}", command);
    println!("{}", product);
    println!("{}", rules);
    println!("{}", final_order);
}
