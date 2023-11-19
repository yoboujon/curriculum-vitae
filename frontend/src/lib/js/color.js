export function shouldColorBeWhite(color) {
    const colorHex = {
        red: parseInt(color.slice(0,2),16),
        green: parseInt(color.slice(2,4),16),
        blue: parseInt(color.slice(4,6),16)
    }
    return (colorHex.red*0.299 + colorHex.green*0.587 + colorHex.blue*0.114) <= 186;
}