import sys
from PIL import Image

def process_image(input_path, output_path):
    img = Image.open(input_path).convert("RGBA")
    
    # Get bounding box of non-transparent pixels
    alpha = img.split()[-1]
    bbox = alpha.getbbox()
    
    if not bbox:
        print("Image is entirely transparent.")
        return

    # Add 2px padding around the non-transparent pixels
    left, upper, right, lower = bbox
    left = max(0, left - 2)
    upper = max(0, upper - 2)
    right = min(img.width, right + 2)
    lower = min(img.height, lower + 2)
    
    img_cropped = img.crop((left, upper, right, lower))
    
    # Make it 1:1 aspect ratio by using the max dimension
    width, height = img_cropped.size
    size = max(width, height)
    
    new_img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    paste_x = (size - width) // 2
    paste_y = (size - height) // 2
    
    new_img.paste(img_cropped, (paste_x, paste_y))
    new_img.save(output_path)
    print(f"Processed image saved to {output_path}")

if __name__ == "__main__":
    process_image("assets/logo/llm-quota-processed.png", "assets/logo/llm-quota-icon.png")
