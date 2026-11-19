fn main() {
    let command = "Generate exactly one ultra-photorealistic real-photo lifestyle product advertisement, square 1000×1000 pixels, clean clear 2K ultra-sharp resolution, indistinguishable from a real photograph to the human naked eye. Use ONLY the exact product from the reference images. Exact colour, exact pattern, exact size, exact proportions. Zero creative input allowed. Absolute precision required.";

    let product = "
    PRODUCT (use ONLY this exact item from the reference images – no changes allowed):
    - White ceramic plates with swirling turquoise / teal interior pattern
    - Concentric spiral design
    - Exact real-life size and proportions
    - Exact colours, glaze finish and spiral pattern from the reference images
    - Do not invent, alter, or improve any detail
    - IMPORTANT: Clearly show both plates together in the same image
    ";

    let rules = "
    The plates must appear as a real photograph, fully visible, razor-sharp, and the absolute dominant subject. Show both plates clearly. Correct real-world scale is mandatory. No packaging.

    SCENE:
    - Clean modern dining table or kitchen counter with soft natural daylight
    - Both plates placed on a light wooden or clean surface at their true real size
    - Show the product only in its natural use as dinner or side plates
    - Extremely clean, premium, uncluttered composition
    - Hands only if placing a plate (no faces or full people)

    STRICT FORBIDDEN:
    - Full faces, full people, or any human presence
    - Extra text, logos, price tags, branding, stickers, crowds, box shots, clutter
    - Showing only one plate
    - Any change to colour, pattern or shape
    - Busy or heavily styled scenes
    - Any artificial or CGI look
    - Any creative input from the AI
    ";

    let final_order = "Final order: Use ONLY the exact white ceramic plates with turquoise spiral interiors from the reference images. Clearly show both plates together. Exact colours, exact pattern, exact size. Ultra-photorealistic, clean clear 2K ultra-sharp, clearly visible and dominant. Highest-quality clean premium dining lifestyle scene. Zero deviation. Zero creative input.";

    println!("{}", command);
    println!("{}", product);
    println!("{}", rules);
    println!("{}", final_order);
}
