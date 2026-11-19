fn main() {
    let command = "Generate exactly one ultra-photorealistic real-photo lifestyle product advertisement, square 1000×1000 pixels, clean clear 2K ultra-sharp resolution, indistinguishable from a real photograph to the human naked eye. Use ONLY the exact product from the reference images. Exact colour, exact fabric texture, exact size, exact proportions. Zero creative input allowed. Absolute precision required.";

    let product = "
    PRODUCT (use ONLY this exact item from the reference images – no changes allowed):
    - Soft grey fitted sheet
    - Smooth, high-quality fabric with natural soft drape
    - Exact real-life size and proportions
    - Exact colour, fabric texture and finish from the reference images
    - Do not invent, alter, or improve any detail
    ";

    let rules = "
    The fitted sheet must appear as a real photograph, fully visible, razor-sharp, and the absolute dominant subject. Correct real-world scale is mandatory. No packaging.

    SCENE:
    - Soft, minimal luxury bedroom with gentle diffused natural daylight
    - Fitted sheet perfectly fitted on a bed with calm, elegant folds
    - High-end, serene atmosphere – soft light, clean lines, premium feel
    - Extremely minimal composition with refined simplicity
    - Optional: hands only gently smoothing the fabric (no faces or full people)
    - Do not place the sheet on top of another sheet
    - Do not show it covering a visible base or truss in an awkward way

    STRICT FORBIDDEN:
    - Full faces, full people, or any human presence beyond hands
    - Sheet stacked on another sheet
    - Awkward coverage of bed base or truss
    - Extra text, logos, price tags, branding, stickers, crowds, box shots, clutter
    - Any change to colour or fabric texture
    - Busy or heavily styled scenes
    - Any artificial or CGI look
    - Any creative input from the AI
    ";

    let final_order = "Final order: Use ONLY the exact soft grey fitted sheet from the reference images. Create a soft, minimal, high-luxury bedroom scene. Exact colour, exact fabric texture, exact size. Ultra-photorealistic, clean clear 2K ultra-sharp, clearly visible and dominant. Highest-quality premium lifestyle scene. Zero deviation. Zero creative input.";

    println!("{}", command);
    println!("{}", product);
    println!("{}", rules);
    println!("{}", final_order);
}
