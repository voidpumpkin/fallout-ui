function _typeof(a){"@babel/helpers - typeof";return _typeof="function"==typeof Symbol&&"symbol"==typeof Symbol.iterator?function(a){return typeof a}:function(a){return a&&"function"==typeof Symbol&&a.constructor===Symbol&&a!==Symbol.prototype?"symbol":typeof a},_typeof(a)}(function(a,b){"function"==typeof define&&define.amd?define([],b):"object"===("undefined"==typeof exports?"undefined":_typeof(exports))?module.exports=b():a.Notifier=b()})(this,function(){var a={autopush:!0,zindex:9999,default_time:4500,vanish_time:300,fps:30,position:"bottom-right",direction:"bottom",success:{classes:"notifyjs-success",textColor:"#155724",borderColor:"#c3e6cb",backgroundColor:"#d4edda",progressColor:"#155724",iconColor:"#155724",icon:"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"8\" height=\"8\" viewBox=\"0 0 8 8\"><path d=\"M6.41 0l-.69.72-2.78 2.78-.81-.78-.72-.72-1.41 1.41.72.72 1.5 1.5.69.72.72-.72 3.5-3.5.72-.72-1.44-1.41z\" transform=\"translate(0 1)\" /></svg>"},error:{classes:"notifyjs-danger",textColor:"#721c24",borderColor:"#f5c6cb",backgroundColor:"#f8d7da",progressColor:"#721c24",iconColor:"#721c24",icon:"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"8\" height=\"8\" viewBox=\"0 0 8 8\"><path d=\"M1.41 0l-1.41 1.41.72.72 1.78 1.81-1.78 1.78-.72.69 1.41 1.44.72-.72 1.81-1.81 1.78 1.81.69.72 1.44-1.44-.72-.69-1.81-1.78 1.81-1.81.72-.72-1.44-1.41-.69.72-1.78 1.78-1.81-1.78-.72-.72z\" /></svg>"},warning:{classes:"notifyjs-warning",textColor:"#856404",borderColor:"#fff3cd",backgroundColor:"#ffeeba",progressColor:"#856404",iconColor:"#856404",icon:"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"8\" height=\"8\" viewBox=\"0 0 8 8\"><path d=\"M3.09 0c-.06 0-.1.04-.13.09l-2.94 6.81c-.02.05-.03.13-.03.19v.81c0 .05.04.09.09.09h6.81c.05 0 .09-.04.09-.09v-.81c0-.05-.01-.14-.03-.19l-2.94-6.81c-.02-.05-.07-.09-.13-.09h-.81zm-.09 3h1v2h-1v-2zm0 3h1v1h-1v-1z\" /></svg>"},info:{classes:"notifyjs-info",textColor:"#0c5460",borderColor:"#d1ecf1",backgroundColor:"#bee5eb",progressColor:"#0c5460",iconColor:"#0c5460",icon:"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"8\" height=\"8\" viewBox=\"0 0 8 8\"><path d=\"M3 0c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm-1.5 2.5c-.83 0-1.5.67-1.5 1.5h1c0-.28.22-.5.5-.5s.5.22.5.5-1 1.64-1 2.5c0 .86.67 1.5 1.5 1.5s1.5-.67 1.5-1.5h-1c0 .28-.22.5-.5.5s-.5-.22-.5-.5c0-.36 1-1.84 1-2.5 0-.81-.67-1.5-1.5-1.5z\" transform=\"translate(2)\"/></svg>"}},b=function(a,b,c,d,e,f,g){switch(this.pushed=!1,this.element=document.createElement("div"),this.element.className=c.classes||"",this.element.style.display="none",this.element.style.position="relative",this.element.style.padding="1em 2em 1em 2.5em",a.options.direction){case"top":this.element.style.marginTop="0.5em";break;case"bottom":default:this.element.style.marginBottom="0.5em";}if(this.element.style.width="100%",this.element.style.borderWidth="1px",this.element.style.borderStyle="solid",this.element.style.borderColor=c.borderColor,this.element.style.boxSizing="border-box",this.element.style.backgroundColor=c.backgroundColor,"undefined"!=typeof c.icon){var h=document.createElement("div");h.style.position="absolute",h.style.top="50%",h.style.left="10px",h.style.transform="translateY(-50%)",h.innerHTML=c.icon,-1==c.icon.indexOf("<img")?-1!=c.icon.indexOf("<svg ")&&(h.childNodes[0].style.width="16px",h.childNodes[0].style.height="16px","undefined"!=typeof c.iconColor&&(h.childNodes[0].style.fill=c.iconColor)):(h.childNodes[0].style.width="16px",h.childNodes[0].style.height="16px"),"undefined"!=typeof c.iconClasses&&(h.childNodes[0].className+=c.iconClasses),this.element.appendChild(h)}var i=document.createElement("span");i.style.color=c.textColor,i.innerHTML=b,this.element.appendChild(i);var j=document.createElement("p");switch(j.className="progress",j.style.position="absolute",j.style.bottom=0,j.style.left=0,j.style.right="100%",j.style.height="2px",j.style.content=" ",j.style.backgroundColor=c.progressColor,j.style.marginBottom=0,this.element.appendChild(j),a.options.direction){case"top":a.container.insertBefore(this.element,a.container.childNodes[0]);break;case"bottom":default:a.container.appendChild(this.element);}this.callback=g;var k=this;this.push=function(){if(!k.pushed){k.pushed=!0;var a=0,b=1e3/f;k.element.style.display="block",k.interval=setInterval(function(){a++;var c=100*(1-b*a/d);j.style.right=c+"%",0>=c&&("function"==typeof g&&g(),k.clear())},b)}},this.clear=function(){if(k.pushed){var a=1e3/f,b=1;"undefined"!=typeof k.interval&&clearInterval(k.interval),k.interval=setInterval(function(){b-=1/(e/a),k.element.style.opacity=b,0>=b&&(clearInterval(k.interval),k.destroy())},a)}},this.destroy=function(){k.pushed&&(k.pushed=!1,"undefined"!=typeof k.interval&&clearInterval(k.interval),a.container.removeChild(k.element))}};return function Notifier(c){if(this.options=Object.assign({},a),this.options=Object.assign(this.options,c),this.container=document.getElementById("notifyjs-container-"+this.options.position),null===this.container){switch(this.container=document.createElement("div"),this.container.id="notifyjs-container-"+this.options.position,this.container.style.zIndex=this.options.zindex,this.container.style.position="fixed",this.container.style.maxWidth="304px",this.container.style.width="100%",this.options.position){case"top-left":this.container.style.top=0,this.container.style.left="0.5em";break;case"top-right":this.container.style.top=0,this.container.style.right="0.5em";break;case"bottom-left":this.container.style.bottom=0,this.container.style.left="0.5em";break;case"bottom-right":default:this.container.style.bottom=0,this.container.style.right="0.5em";}document.getElementsByTagName("body")[0].appendChild(this.container)}this.notify=function(a,c,d,e){if("undefined"==typeof this.options[a])return void console.error("Notify.js: Error, undefined '"+a+"' notification type");"undefined"==typeof d&&(d=this.options.default_time);var f=new b(this,c,this.options[a],d,this.options.vanish_time,this.options.fps,e);return this.options.autopush&&f.push(),f}}});