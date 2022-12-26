'use strict';
/* eslint-disable */

let links = new Array(new Array("Azurlane Tech Research  碧蓝航线科研规划", "./techresearch/techresearch.html"));
function resizer() {
    let width = document.documentElement.clientWidth;
    // var height = document.documentElement.clientHeight;
    let uldiv = document.querySelector("body div div.listgp");
    let ulist = uldiv.childNodes;
    for (var i=0;i<ulist.length;i++) {
        uldiv.removeChild(ulist[i]);
    }
    let dat = [...links];
    let res;
    if (dat.length === 1) {
        res = [dat];
    } else {
        if (width <= 768) {
            res = [dat];
        } else if (width >= 1200 && dat.length !== 2) {
            res = [dat.splice(0, Math.ceil(dat.length / 3)), dat.splice(0, Math.ceil(dat.length / 2)), a];
        } else {
            res = [dat.splice(0, Math.ceil(dat.length / 2)), dat];
        }
    }
    let nul, nli, na;
    for (var i=0;i<res.length;i++) {
        nul = document.createElement('ul');
        nul.classList.add('list-group')
        nul.classList.add('col-12')
        nul.classList.add('col-md-6')
        nul.classList.add('col-xl-4');
        for (var j=0;j<res[i].length;j++) {
            nli = document.createElement('li');
            nli.classList.add("list-group-item");
            na = document.createElement('a');
            na.setAttribute('href', res[i][j][1]);
            na.textContent = res[i][j][0];
            nli.appendChild(na);
        }
        nul.appendChild(nli);
        uldiv.appendChild(nul);
    }
}

window.addEventListener('resize', resizer);
// resizer();
window.onload = resizer;