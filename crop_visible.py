# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "Pillow",
# ]
# ///

from PIL import Image

input_path = 'assets/logo/llm-quota-processed.png'
output_path = 'assets/logo/llm-quota-tight.png'

print(f"Processing {input_path}...")
img = Image.open(input_path).convert("RGBA")

# Get bounding box of non-zero alpha
bbox = img.getbbox()

if bbox:
    print(f"Original bounding box: {bbox}")
    # bbox is (left, upper, right, lower)
    # Add 2px padding
    left = max(0, bbox[0] - 2)
    upper = max(0, bbox[1] - 2)
    right = min(img.width, bbox[2] + 2)
    lower = min(img.height, bbox[3] + 2)
    
    # We want a 1:1 image according to the previous task context, or just tightly cropped?
    # "cut out that visible image keeping 2px in all side" implies tight crop.
    # But usually app icons need to be square. Let's make it tightly cropped, but if it's not square, 
    # we pad the shorter side to make it square again, keeping the 2px padding on the tightest edges.
    
    width = right - left
    height = lower - upper
    
    if width != height:
        size = max(width, height)
        # Calculate new bounds to make it square
        diff_w = size - width
        diff_h = size - height
        
        left = max(0, left - diff_w // 2)
        upper = max(0, upper - diff_h // 2)
        right = left + size
        lower = upper + size
    
    img_cropped = img.crop((left, upper, right, lower))
    
    img_cropped.save(output_path)
    print(f"Saved tightly cropped 1:1 image to {output_path}")
else:
    print("Image is entirely transparent!")
