(this.webpackJsonpapp=this.webpackJsonpapp||[]).push([[0],{22:function(e,n,t){},24:function(e,n,t){},30:function(e,n,t){"use strict";t.r(n);var c=t(0),r=t.n(c),s=t(15),i=t.n(s),d=(t(22),t(2)),j=t(13),o=t(9),a=t(1),b=function(){var e=r.a.useState(),n=Object(o.a)(e,2),t=n[0],c=n[1];return r.a.useEffect((function(){window.electron.ipcRenderer.on("return",(function(e){c(e)}))}),[]),Object(a.jsx)("div",{children:t?Object(a.jsxs)("div",{children:["ID: ",t.id,Object(a.jsx)("br",{}),"State: ",t.state,Object(a.jsx)("br",{}),"Local: ",t.return_address,Object(a.jsx)("br",{}),"Remote: ",t.send_address,Object(a.jsx)("br",{}),"SynSent"===t.state&&Object(a.jsx)("button",{onClick:function(){window.electron.ipcRenderer.send("return","ack",t.id)},children:"Ack"})]}):Object(a.jsx)("button",{onClick:function(){window.electron.ipcRenderer.send("return","listen")},children:"Connect"})})},u=function(){var e=r.a.useState(),n=Object(o.a)(e,2),t=n[0],c=n[1],s=r.a.useState(""),i=Object(o.a)(s,2),d=i[0],j=i[1];return r.a.useEffect((function(){window.electron.ipcRenderer.on("send",(function(e){c(e)}))}),[]),Object(a.jsx)("div",{children:t?Object(a.jsxs)("div",{children:["ID: ",t.id,Object(a.jsx)("br",{}),"State: ",t.state,Object(a.jsx)("br",{}),"Local: ",t.send_address,Object(a.jsx)("br",{}),"Remote: ",t.return_address,Object(a.jsx)("br",{}),"SynAcked"===t.state&&Object(a.jsx)("button",{onClick:function(){window.electron.ipcRenderer.send("send","stream",t)},children:"Stream"})]}):Object(a.jsxs)(a.Fragment,{children:[Object(a.jsx)("input",{onChange:function(e){return j(e.target.value)},value:d}),Object(a.jsx)("button",{onClick:function(){window.electron.ipcRenderer.send("send","syn",d)},children:"Connect"})]})})};t(24);var l=function(){return Object(a.jsx)("div",{className:"app",children:Object(a.jsxs)(d.a,{children:[Object(a.jsxs)("div",{className:"app-menu",children:[Object(a.jsx)(j.a,{to:"/return",children:"Return"}),Object(a.jsx)(j.a,{to:"/send",children:"Send"})]}),Object(a.jsxs)(d.d,{children:[Object(a.jsx)(d.b,{path:"/",exact:!0,children:"Home"}),Object(a.jsx)(d.b,{path:"/return",component:b}),Object(a.jsx)(d.b,{path:"/send",component:u})]})]})})},O=function(e){e&&e instanceof Function&&t.e(3).then(t.bind(null,31)).then((function(n){var t=n.getCLS,c=n.getFID,r=n.getFCP,s=n.getLCP,i=n.getTTFB;t(e),c(e),r(e),s(e),i(e)}))};i.a.render(Object(a.jsx)(r.a.StrictMode,{children:Object(a.jsx)(l,{})}),document.getElementById("root")),O()}},[[30,1,2]]]);
//# sourceMappingURL=main.c5a8268d.chunk.js.map