fn main() {
    let command = "Generate exactly one ultra-photorealistic real-photo lifestyle product advertisement, square 1000×1000 pixels, clean clear 2K ultra-sharp resolution, indistinguishable from a real photograph to the human naked eye. Use ONLY the exact product from the reference images. Exact colour, exact texture, exact size, exact proportions. Zero creative input allowed. Absolute precision required.";

    let product = "
    PRODUCT (use ONLY this exact item from the reference images – no changes allowed):
    - Chef’s knife with dark wooden handle and dark stainless steel blade
    - Textured / forged blade finish
    - Three silver rivets on the handle
    - Exact real-life size and proportions
    - Exact handle wood grain, blade colour, texture and shape from the reference images
    - Do not invent, alter, or improve any detail
    ";

    let rules = "
    The knife must appear as a real photograph, fully visible, razor-sharp, and the absolute dominant subject. Correct real-world scale is mandatory. No packaging.

    SCENE:
    - Clean modern kitchen with soft natural daylight
    - Hands only (no faces or full people) using the knife to cut fresh vegetables on a light wooden cutting board
    - Vegetables such as cucumber, carrot or similar neatly arranged
    - Extremely clean, premium, uncluttered composition
    - Product remains clearly visible and dominant while in use

    STRICT FORBIDDEN:
    - Full faces, full people, or multiple people
    - Extra text, logos, price tags, branding, stickers, crowds, box shots, clutter
    - Any change to blade colour, handle wood, texture or shape
    - Busy or heavily styled scenes
    - Any artificial or CGI look
    - Any creative input from the AI
    ";

    let final_order = "Final order: Use ONLY the exact dark-handled chef’s knife from the reference images. Show hands only using the knife to cut vegetables on a clean wooden board. Exact colour, exact texture, exact size. Ultra-photorealistic, clean clear 2K ultra-sharp, clearly visible and dominant. Highest-quality clean premium kitchen lifestyle scene. Zero deviation. Zero creative input.";

    println!("{}", command);
    println!("{}", product);
    println!("{}", rules);
    println!("{}", final_order);
}
