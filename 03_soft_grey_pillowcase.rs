fn main() {
    let command = "Generate exactly one ultra-photorealistic real-photo lifestyle product advertisement, square 1000×1000 pixels, clean clear 2K ultra-sharp resolution, indistinguishable from a real photograph to the human naked eye. Use ONLY the exact product from the reference images. Exact colour, exact fabric texture, exact size, exact proportions. Zero creative input allowed. Absolute precision required.";

    let product = "
    PRODUCT (use ONLY this exact item from the reference images – no changes allowed):
    - Soft grey pillowcase with button closure
    - Smooth high-quality fabric with natural drape
    - Exact real-life size and proportions
    - Exact colour, fabric texture, sheen and button detail from the reference images
    - Do not invent, alter, or improve any detail
    ";

    let rules = "
    The pillowcase must appear as a real photograph, fully visible, razor-sharp, and the absolute dominant subject. Correct real-world scale is mandatory. No packaging.

    SCENE:
    - Soft minimal luxury bedroom with gentle diffused natural daylight
    - Single pillowcase neatly placed on a bed or clean surface
    - High-end, serene atmosphere with soft light and refined simplicity
    - Extremely minimal composition
    - Optional: hands only gently adjusting the pillowcase (no faces or full people)

    STRICT FORBIDDEN:
    - Full faces, full people, or any human presence beyond hands
    - Extra text, logos, price tags, branding, stickers, crowds, box shots, clutter
    - Any change to colour or fabric texture
    - Busy or heavily styled scenes
    - Any artificial or CGI look
    - Any creative input from the AI
    ";

    let final_order = "Final order: Use ONLY the exact soft grey pillowcase from the reference images. Create a soft, minimal, high-luxury bedroom scene. Exact colour, exact fabric texture, exact size. Ultra-photorealistic, clean clear 2K ultra-sharp, clearly visible and dominant. Highest-quality premium lifestyle scene. Zero deviation. Zero creative input.";

    println!("{}", command);
    println!("{}", product);
    println!("{}", rules);
    println!("{}", final_order);
}
