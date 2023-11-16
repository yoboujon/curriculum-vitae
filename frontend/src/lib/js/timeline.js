export function createTimeLine(positionsArray,typename) {
    let divArray = []
    console.log(positionsArray);
    for(let i=0; i<positionsArray.length-1; i++)
    {
        var newDiv = document.createElement('div');
        newDiv.className = `${typename}-string`;
        const left = positionsArray[i].left + (2.5*16);
        newDiv.style.left = `${left}px`;
        const top = positionsArray[i].top + 16;
        newDiv.style.top = `${top}px`;
        const width = ((positionsArray[i+1].left)-(positionsArray[i].left));
        console.log(width);
        newDiv.style.width = `${width}px`;
        divArray.push(newDiv);
    }
    return divArray;
}