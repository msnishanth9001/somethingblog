var consoleLogArray = [];

console.log = function (message) {
    consoleLogArray.push(message);
    originalConsoleLog.apply(console, arguments);
};

var originalConsoleLog = console.log;

function displayItems() {
    var itemList = document.getElementById("itemList");
    
    consoleLogArray.forEach(function (item) {
        var li = document.createElement("li");
        li.textContent = item;
        itemList.appendChild(li);
    });
}

window.addEventListener("load", function () {
    displayItems();
});
