# How to Adjust Bottom Panel Height

## Quick Guide

To change the height of the bottom panel (Terminal, LSP, Runner), edit this file:

**File:** `src/components/Main.svelte`

**Line to change:** Look for this section in the `<style>` block:

```css
#main { 
  display: grid; 
  grid-template-rows: 1fr 350px;  /* <-- CHANGE THIS VALUE */
  background: var(--bg2); 
  overflow: hidden;
  transition: grid-template-rows 0.3s ease;
}
```

## Adjustment Options

### Smaller Panel (Less Space)
```css
grid-template-rows: 1fr 250px;  /* Small - good for quick checks */
```

### Default (Current)
```css
grid-template-rows: 1fr 350px;  /* Medium - balanced */
```

### Larger Panel (More Space)
```css
grid-template-rows: 1fr 450px;  /* Large - better for terminal work */
```

### Extra Large
```css
grid-template-rows: 1fr 550px;  /* Extra large - half screen */
```

## What Was Fixed

1. ✅ **Toggle now properly collapses** - Added CSS rule for `.bottom-panel-collapsed` class
2. ✅ **Auto-scroll when expanding** - Panel scrolls into view when you expand it
3. ✅ **Smooth transitions** - Added smooth animation when toggling
4. ✅ **Increased default height** - Changed from 260px to 350px for better visibility

## Tips

- The first value `1fr` is for the editor area (takes remaining space)
- The second value (e.g., `350px`) is the bottom panel height
- Keep values between `200px` - `600px` for best results
- Changes take effect immediately when you save the file
