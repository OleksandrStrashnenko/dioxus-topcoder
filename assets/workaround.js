// workaround of bug not firing drag events
function setDataToDraggables()  {
    let draggables = document.querySelectorAll('[draggable="true"]');
    for (const draggable of draggables) {
        draggable.addEventListener("dragstart", (e) => {
            e.dataTransfer.setData("text/plain", e.target.id);
        });
    }
}