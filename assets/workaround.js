// workaround of bug not firing drag events

for (var draggable of document.querySelectorAll('[draggable="true"]')) {
    draggable.addEventListener("dragstart", (e) => {
        e.dataTransfer.setData("", "")
    });
}