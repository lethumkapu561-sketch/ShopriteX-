fn main() {
    let command = "Generate exactly one ultra-photorealistic real-photo lifestyle product advertisement, square 1000×1000 pixels, clean clear 2K ultra-sharp resolution, indistinguishable from a real photograph to the human naked eye. Use ONLY the exact product from the reference images. Exact colour, exact size, exact proportions. Zero creative input allowed. Absolute precision required.";

    let product = "
    PRODUCT (use ONLY this exact item from the reference images – no changes allowed):
    - Matte black ceramic mug
    - Modern tapered shape with rectangular handle
    - Exact real-life size and proportions
    - Exact colour, matte finish and handle shape from the reference images
    - Do not invent, alter, or improve any detail
    ";

    let rules = "
    The mug must appear as a real photograph, fully visible, razor-sharp, and the absolute dominant subject. Correct real-world scale is mandatory. No packaging.

    SCENE:
    - Clean modern kitchen or breakfast table with soft natural daylight
    - Mug placed on a light wooden or clean surface at its true real size
    - Show the product only in its natural use as a drinking mug
    - Hands only if holding or placing the mug (no faces or full people)
    - Extremely clean, uncluttered and premium composition

    STRICT FORBIDDEN:
    - Full faces, full people, or multiple people
    - Extra text, logos, price tags, branding, stickers, crowds, box shots, clutter
    - Any change to colour, size or shape
    - Any artificial or CGI look
    - Any creative input from the AI
    ";

    let final_order = "Final order: Use ONLY the exact matte black ceramic mug from the reference images. Exact colour, exact size. Ultra-photorealistic, clean clear 2K ultra-sharp, clearly visible and dominant. Highest-quality clean premium kitchen lifestyle scene. Zero deviation. Zero creative input.";

    println!("{}", command);
    println!("{}", product);
    println!("{}", rules);
    println!("{}", final_order);
}
