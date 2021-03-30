import * as wasm from "wasm-summarize";
console.log("Hello");
console.log(wasm.add(9,12));
//console.log(wasm.markdown_extract("currently, __this__ is the *main* feature"))

let animationId = null;

const isPaused = () => {
    return animationId === null;
  };

  
const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  console.log("play");
  animationId = 3;
};

const pause = () => {
  playPauseButton.textContent = "▶";
  console.log("pause");
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});


//prevent showing of query in url
window.history.pushState({}, document.title, "/");

const userinput = document.getElementById("form_summarize");

// prevent reloading
function handleForm(event) { event.preventDefault(); } 
userinput.addEventListener('submit', handleForm);

//run wasm
userinput.addEventListener("submit", event => {
    console.log("hi");
    //document.forms[0].summarize_content.value.toString()
    let user_content = document.getElementById("summarize_content").value.toString();
    console.log("Clicked: "+user_content);
    let result = wasm.markdown_extract(user_content);
    console.log(result);
    document.getElementById("extract").innerHTML = result;
    /*
    var words;
    for (words in result) {
      document.getElementById("extract").innerHTML = words;
    }*/
})
/*

import 'd3';

var layout = d3.layout.cloud()
    .size([500, 500])
    .words([
      "Hello", "world", "normally", "you", "want", "more", "words",
      "than", "this"].map(function(d) {
      return {text: d, size: 10 + Math.random() * 90, test: "haha"};
    }))
    .padding(5)
    .rotate(function() { return ~~(Math.random() * 2) * 90; })
    .font("Impact")
    .fontSize(function(d) { return d.size; })
    .on("end", draw);

d3.layout.start();
function draw(words) {
  d3.select("body").append("svg")
      .attr("width", layout.size()[0])
      .attr("height", layout.size()[1])
    .append("g")
      .attr("transform", "translate(" + layout.size()[0] / 2 + "," + layout.size()[1] / 2 + ")")
    .selectAll("text")
      .data(words)
    .enter().append("text")
      .style("font-size", function(d) { return d.size + "px"; })
      .style("font-family", "Impact")
      .attr("text-anchor", "middle")
      .attr("transform", function(d) {
        return "translate(" + [d.x, d.y] + ")rotate(" + d.rotate + ")";
      })
      .text(function(d) { return d.text; });
}
*/