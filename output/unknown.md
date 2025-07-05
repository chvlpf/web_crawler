

if(navigator.userAgent.match(/MSIE|Internet Explorer/i)||navigator.userAgent.match(/Trident\/7\..*?rv:11/i)){var href=document.location.href;if(!href.match(/[?&]nowprocket/)){if(href.indexOf("?")==-1){if(href.indexOf("#")==-1){document.location.href=href+"?nowprocket=1"}else{document.location.href=href.replace("#","?nowprocket=1#")}}else{if(href.indexOf("#")==-1){document.location.href=href+"&nowprocket=1"}else{document.location.href=href.replace("#","&nowprocket=1#")}}}}class RocketLazyLoadScripts{constructor(){this.v="1.2.5.1",this.triggerEvents=["keydown","mousedown","mousemove","touchmove","touchstart","touchend","wheel"],this.userEventHandler=this._triggerListener.bind(this),this.touchStartHandler=this._onTouchStart.bind(this),this.touchMoveHandler=this._onTouchMove.bind(this),this.touchEndHandler=this._onTouchEnd.bind(this),this.clickHandler=this._onClick.bind(this),this.interceptedClicks=[],this.interceptedClickListeners=[],this._interceptClickListeners(this),window.addEventListener("pageshow",e={this.persisted=e.persisted,this.everythingLoaded&&this._triggerLastFunctions()}),document.addEventListener("DOMContentLoaded",()={this._preconnect3rdParties()}),this.delayedScripts={normal:[],async:[],defer:[]},this.trash=[],this.allJQueries=[]}_addUserInteractionListener(e){if(document.hidden){e._triggerListener();return}this.triggerEvents.forEach(t=window.addEventListener(t,e.userEventHandler,{passive:!0})),window.addEventListener("touchstart",e.touchStartHandler,{passive:!0}),window.addEventListener("mousedown",e.touchStartHandler),document.addEventListener("visibilitychange",e.userEventHandler)}_removeUserInteractionListener(){this.triggerEvents.forEach(e=window.removeEventListener(e,this.userEventHandler,{passive:!0})),document.removeEventListener("visibilitychange",this.userEventHandler)}_onTouchStart(e){"HTML"!==e.target.tagName&&(window.addEventListener("touchend",this.touchEndHandler),window.addEventListener("mouseup",this.touchEndHandler),window.addEventListener("touchmove",this.touchMoveHandler,{passive:!0}),window.addEventListener("mousemove",this.touchMoveHandler),e.target.addEventListener("click",this.clickHandler),this._disableOtherEventListeners(e.target,!0),this._renameDOMAttribute(e.target,"onclick","rocket-onclick"),this._pendingClickStarted())}_onTouchMove(e){window.removeEventListener("touchend",this.touchEndHandler),window.removeEventListener("mouseup",this.touchEndHandler),window.removeEventListener("touchmove",this.touchMoveHandler,{passive:!0}),window.removeEventListener("mousemove",this.touchMoveHandler),e.target.removeEventListener("click",this.clickHandler),this._disableOtherEventListeners(e.target,!1),this._renameDOMAttribute(e.target,"rocket-onclick","onclick"),this._pendingClickFinished()}_onTouchEnd(){window.removeEventListener("touchend",this.touchEndHandler),window.removeEventListener("mouseup",this.touchEndHandler),window.removeEventListener("touchmove",this.touchMoveHandler,{passive:!0}),window.removeEventListener("mousemove",this.touchMoveHandler)}_onClick(e){e.target.removeEventListener("click",this.clickHandler),this._disableOtherEventListeners(e.target,!1),this._renameDOMAttribute(e.target,"rocket-onclick","onclick"),this.interceptedClicks.push(e),e.preventDefault(),e.stopPropagation(),e.stopImmediatePropagation(),this._pendingClickFinished()}_replayClicks(){window.removeEventListener("touchstart",this.touchStartHandler,{passive:!0}),window.removeEventListener("mousedown",this.touchStartHandler),this.interceptedClicks.forEach(e={e.target.dispatchEvent(new MouseEvent("click",{view:e.view,bubbles:!0,cancelable:!0}))})}_interceptClickListeners(e){EventTarget.prototype.addEventListenerBase=EventTarget.prototype.addEventListener,EventTarget.prototype.addEventListener=function(t,i,r){"click"!==t||e.windowLoaded||i===e.clickHandler||e.interceptedClickListeners.push({target:this,func:i,options:r}),(this||window).addEventListenerBase(t,i,r)}}_disableOtherEventListeners(e,t){this.interceptedClickListeners.forEach(i={i.target===e&&(t?e.removeEventListener("click",i.func,i.options):e.addEventListener("click",i.func,i.options))}),e.parentNode!==document.documentElement&&this._disableOtherEventListeners(e.parentNode,t)}_waitForPendingClicks(){return new Promise(e={this._isClickPending?this._pendingClickFinished=e:e()})}_pendingClickStarted(){this._isClickPending=!0}_pendingClickFinished(){this._isClickPending=!1}_renameDOMAttribute(e,t,i){e.hasAttribute&&e.hasAttribute(t)&&(event.target.setAttribute(i,event.target.getAttribute(t)),event.target.removeAttribute(t))}_triggerListener(){this._removeUserInteractionListener(this),"loading"===document.readyState?document.addEventListener("DOMContentLoaded",this._loadEverythingNow.bind(this)):this._loadEverythingNow()}_preconnect3rdParties(){let e=[];document.querySelectorAll("script[type=rocketlazyloadscript][data-rocket-src]").forEach(t={let i=t.getAttribute("data-rocket-src");if(i&&0!==i.indexOf("data:")){0===i.indexOf("//")&&(i=location.protocol+i);try{let r=new URL(i).origin;r!==location.origin&&e.push({src:r,crossOrigin:t.crossOrigin||"module"===t.getAttribute("data-rocket-type")})}catch(n){}}}),e=[...new Map(e.map(e=[JSON.stringify(e),e])).values()],this._batchInjectResourceHints(e,"preconnect")}async _loadEverythingNow(){this.lastBreath=Date.now(),this._delayEventListeners(this),this._delayJQueryReady(this),this._handleDocumentWrite(),this._registerAllDelayedScripts(),this._preloadAllScripts(),await this._loadScriptsFromList(this.delayedScripts.normal),await this._loadScriptsFromList(this.delayedScripts.defer),await this._loadScriptsFromList(this.delayedScripts.async);try{await this._triggerDOMContentLoaded(),await this._pendingWebpackRequests(this),await this._triggerWindowLoad()}catch(e){console.error(e)}window.dispatchEvent(new Event("rocket-allScriptsLoaded")),this.everythingLoaded=!0,this._waitForPendingClicks().then(()={this._replayClicks()}),this._emptyTrash()}_registerAllDelayedScripts(){document.querySelectorAll("script[type=rocketlazyloadscript]").forEach(e={e.hasAttribute("data-rocket-src")?e.hasAttribute("async")&&!1!==e.async?this.delayedScripts.async.push(e):e.hasAttribute("defer")&&!1!==e.defer||"module"===e.getAttribute("data-rocket-type")?this.delayedScripts.defer.push(e):this.delayedScripts.normal.push(e):this.delayedScripts.normal.push(e)})}async _transformScript(e){if(await this._littleBreath(),!0===e.noModule&&"noModule"in HTMLScriptElement.prototype){e.setAttribute("data-rocket-status","skipped");return}return new Promise(t={let i;function r(){(i||e).setAttribute("data-rocket-status","executed"),t()}try{if(navigator.userAgent.indexOf("Firefox/")0||""===navigator.vendor)i=document.createElement("script"),[...e.attributes].forEach(e={let t=e.nodeName;"type"!==t&&("data-rocket-type"===t&&(t="type"),"data-rocket-src"===t&&(t="src"),i.setAttribute(t,e.nodeValue))}),e.text&&(i.text=e.text),i.hasAttribute("src")?(i.addEventListener("load",r),i.addEventListener("error",function(){i.setAttribute("data-rocket-status","failed"),t()}),setTimeout(()={i.isConnected||t()},1)):(i.text=e.text,r()),e.parentNode.replaceChild(i,e);else{let n=e.getAttribute("data-rocket-type"),s=e.getAttribute("data-rocket-src");n?(e.type=n,e.removeAttribute("data-rocket-type")):e.removeAttribute("type"),e.addEventListener("load",r),e.addEventListener("error",function(){e.setAttribute("data-rocket-status","failed"),t()}),s?(e.removeAttribute("data-rocket-src"),e.src=s):e.src="data:text/javascript;base64,"+window.btoa(unescape(encodeURIComponent(e.text)))}}catch(a){e.setAttribute("data-rocket-status","failed"),t()}})}async _loadScriptsFromList(e){let t=e.shift();return t&&t.isConnected?(await this._transformScript(t),this._loadScriptsFromList(e)):Promise.resolve()}_preloadAllScripts(){this._batchInjectResourceHints([...this.delayedScripts.normal,...this.delayedScripts.defer,...this.delayedScripts.async],"preload")}_batchInjectResourceHints(e,t){var i=document.createDocumentFragment();e.forEach(e={let r=e.getAttribute&&e.getAttribute("data-rocket-src")||e.src;if(r){let n=document.createElement("link");n.href=r,n.rel=t,"preconnect"!==t&&(n.as="script"),e.getAttribute&&"module"===e.getAttribute("data-rocket-type")&&(n.crossOrigin=!0),e.crossOrigin&&(n.crossOrigin=e.crossOrigin),e.integrity&&(n.integrity=e.integrity),i.appendChild(n),this.trash.push(n)}}),document.head.appendChild(i)}_delayEventListeners(e){let t={};function i(i,r){return t[r].eventsToRewrite.indexOf(i)=0&&!e.everythingLoaded?"rocket-"+i:i}function r(e,r){var n;!t[n=e]&&(t[n]={originalFunctions:{add:n.addEventListener,remove:n.removeEventListener},eventsToRewrite:[]},n.addEventListener=function(){arguments[0]=i(arguments[0],n),t[n].originalFunctions.add.apply(n,arguments)},n.removeEventListener=function(){arguments[0]=i(arguments[0],n),t[n].originalFunctions.remove.apply(n,arguments)}),t[e].eventsToRewrite.push(r)}function n(t,i){let r=t[i];t[i]=null,Object.defineProperty(t,i,{get:()=r||function(){},set(n){e.everythingLoaded?r=n:t["rocket"+i]=r=n}})}r(document,"DOMContentLoaded"),r(window,"DOMContentLoaded"),r(window,"load"),r(window,"pageshow"),r(document,"readystatechange"),n(document,"onreadystatechange"),n(window,"onload"),n(window,"onpageshow")}_delayJQueryReady(e){let t;function i(t){return e.everythingLoaded?t:t.split(" ").map(e="load"===e||0===e.indexOf("load.")?"rocket-jquery-load":e).join(" ")}function r(r){if(r&&r.fn&&!e.allJQueries.includes(r)){r.fn.ready=r.fn.init.prototype.ready=function(t){return e.domReadyFired?t.bind(document)(r):document.addEventListener("rocket-DOMContentLoaded",()=t.bind(document)(r)),r([])};let n=r.fn.on;r.fn.on=r.fn.init.prototype.on=function(){return this[0]===window&&("string"==typeof arguments[0]||arguments[0]instanceof String?arguments[0]=i(arguments[0]):"object"==typeof arguments[0]&&Object.keys(arguments[0]).forEach(e={let t=arguments[0][e];delete arguments[0][e],arguments[0][i(e)]=t})),n.apply(this,arguments),this},e.allJQueries.push(r)}t=r}r(window.jQuery),Object.defineProperty(window,"jQuery",{get:()=t,set(e){r(e)}})}async _pendingWebpackRequests(e){let t=document.querySelector("script[data-webpack]");async function i(){return new Promise(e={t.addEventListener("load",e),t.addEventListener("error",e)})}t&&(await i(),await e._requestAnimFrame(),await e._pendingWebpackRequests(e))}async _triggerDOMContentLoaded(){this.domReadyFired=!0,await this._littleBreath(),document.dispatchEvent(new Event("rocket-readystatechange")),await this._littleBreath(),document.rocketonreadystatechange&&document.rocketonreadystatechange(),await this._littleBreath(),document.dispatchEvent(new Event("rocket-DOMContentLoaded")),await this._littleBreath(),window.dispatchEvent(new Event("rocket-DOMContentLoaded"))}async _triggerWindowLoad(){await this._littleBreath(),document.dispatchEvent(new Event("rocket-readystatechange")),await this._littleBreath(),document.rocketonreadystatechange&&document.rocketonreadystatechange(),await this._littleBreath(),window.dispatchEvent(new Event("rocket-load")),await this._littleBreath(),window.rocketonload&&window.rocketonload(),await this._littleBreath(),this.allJQueries.forEach(e=e(window).trigger("rocket-jquery-load")),await this._littleBreath();let e=new Event("rocket-pageshow");e.persisted=this.persisted,window.dispatchEvent(e),await this._littleBreath(),window.rocketonpageshow&&window.rocketonpageshow({persisted:this.persisted}),this.windowLoaded=!0}_triggerLastFunctions(){document.onreadystatechange&&document.onreadystatechange(),window.onload&&window.onload(),window.onpageshow&&window.onpageshow({persisted:this.persisted})}_handleDocumentWrite(){let e=new Map;document.write=document.writeln=function(t){let i=document.currentScript;i||console.error("WPRocket unable to document.write this: "+t);let r=document.createRange(),n=i.parentElement,s=e.get(i);void 0===s&&(s=i.nextSibling,e.set(i,s));let a=document.createDocumentFragment();r.setStart(a,0),a.appendChild(r.createContextualFragment(t)),n.insertBefore(a,s)}}async _littleBreath(){Date.now()-this.lastBreath45&&(await this._requestAnimFrame(),this.lastBreath=Date.now())}async _requestAnimFrame(){return document.hidden?new Promise(e=setTimeout(e)):new Promise(e=requestAnimationFrame(e))}_emptyTrash(){this.trash.forEach(e=e.remove())}static run(){let e=new RocketLazyLoadScripts;e._addUserInteractionListener(e)}}RocketLazyLoadScripts.run();



	img:is([sizes="auto" i], [sizes^="auto," i]) { contain-intrinsic-size: 3000px 1500px }
	

heygoody.com โบรกเกอร์ประกันออนไลน์ ซื้อง่าย เคลมได้จริง!!





























{"@context":"https://schema.org","@graph":[{"@type":"BreadcrumbList","@id":"https://www.heygoody.com/th/home/#breadcrumb","itemListElement":[{"@type":"ListItem","position":"1","item":{"@id":"https://www.heygoody.com/th/home","name":"\u0e2b\u0e19\u0e49\u0e32\u0e2b\u0e25\u0e31\u0e01"}},{"@type":"ListItem","position":"2","item":{"@id":"https://www.heygoody.com/th/home/","name":"Home"}}]}]}






























:root{--wp-admin-theme-color:#007cba;--wp-admin-theme-color--rgb:0,124,186;--wp-admin-theme-color-darker-10:#006ba1;--wp-admin-theme-color-darker-10--rgb:0,107,161;--wp-admin-theme-color-darker-20:#005a87;--wp-admin-theme-color-darker-20--rgb:0,90,135;--wp-admin-border-width-focus:2px;--wp-block-synced-color:#7a00df;--wp-block-synced-color--rgb:122,0,223;--wp-bound-block-color:var(--wp-block-synced-color)}@media (min-resolution:192dpi){:root{--wp-admin-border-width-focus:1.5px}}.wp-element-button{cursor:pointer}:root{--wp--preset--font-size--normal:16px;--wp--preset--font-size--huge:42px}:root .has-very-light-gray-background-color{background-color:#eee}:root .has-very-dark-gray-background-color{background-color:#313131}:root .has-very-light-gray-color{color:#eee}:root .has-very-dark-gray-color{color:#313131}:root .has-vivid-green-cyan-to-vivid-cyan-blue-gradient-background{background:linear-gradient(135deg,#00d084,#0693e3)}:root .has-purple-crush-gradient-background{background:linear-gradient(135deg,#34e2e4,#4721fb 50%,#ab1dfe)}:root .has-hazy-dawn-gradient-background{background:linear-gradient(135deg,#faaca8,#dad0ec)}:root .has-subdued-olive-gradient-background{background:linear-gradient(135deg,#fafae1,#67a671)}:root .has-atomic-cream-gradient-background{background:linear-gradient(135deg,#fdd79a,#004a59)}:root .has-nightshade-gradient-background{background:linear-gradient(135deg,#330968,#31cdcf)}:root .has-midnight-gradient-background{background:linear-gradient(135deg,#020381,#2874fc)}.has-regular-font-size{font-size:1em}.has-larger-font-size{font-size:2.625em}.has-normal-font-size{font-size:var(--wp--preset--font-size--normal)}.has-huge-font-size{font-size:var(--wp--preset--font-size--huge)}.has-text-align-center{text-align:center}.has-text-align-left{text-align:left}.has-text-align-right{text-align:right}#end-resizable-editor-section{display:none}.aligncenter{clear:both}.items-justified-left{justify-content:flex-start}.items-justified-center{justify-content:center}.items-justified-right{justify-content:flex-end}.items-justified-space-between{justify-content:space-between}.screen-reader-text{border:0;clip:rect(1px,1px,1px,1px);clip-path:inset(50%);height:1px;margin:-1px;overflow:hidden;padding:0;position:absolute;width:1px;word-wrap:normal!important}.screen-reader-text:focus{background-color:#ddd;clip:auto!important;clip-path:none;color:#444;display:block;font-size:1em;height:auto;left:5px;line-height:normal;padding:15px 23px 14px;text-decoration:none;top:5px;width:auto;z-index:100000}html :where(.has-border-color){border-style:solid}html :where([style*=border-top-color]){border-top-style:solid}html :where([style*=border-right-color]){border-right-style:solid}html :where([style*=border-bottom-color]){border-bottom-style:solid}html :where([style*=border-left-color]){border-left-style:solid}html :where([style*=border-width]){border-style:solid}html :where([style*=border-top-width]){border-top-style:solid}html :where([style*=border-right-width]){border-right-style:solid}html :where([style*=border-bottom-width]){border-bottom-style:solid}html :where([style*=border-left-width]){border-left-style:solid}html :where(img[class*=wp-image-]){height:auto;max-width:100%}:where(figure){margin:0 0 1em}html :where(.is-position-sticky){--wp-admin--admin-bar--position-offset:var(--wp-admin--admin-bar--height,0px)}@media screen and (max-width:600px){html :where(.is-position-sticky){--wp-admin--admin-bar--position-offset:0px}}


/*! This file is auto-generated */
.wp-block-button__link{color:#fff;background-color:#32373c;border-radius:9999px;box-shadow:none;text-decoration:none;padding:calc(.667em + 2px) calc(1.333em + 2px);font-size:1.125em}.wp-block-file__button{background:#32373c;color:#fff;text-decoration:none}


:root{--wp--preset--aspect-ratio--square: 1;--wp--preset--aspect-ratio--4-3: 4/3;--wp--preset--aspect-ratio--3-4: 3/4;--wp--preset--aspect-ratio--3-2: 3/2;--wp--preset--aspect-ratio--2-3: 2/3;--wp--preset--aspect-ratio--16-9: 16/9;--wp--preset--aspect-ratio--9-16: 9/16;--wp--preset--color--black: #000000;--wp--preset--color--cyan-bluish-gray: #abb8c3;--wp--preset--color--white: #ffffff;--wp--preset--color--pale-pink: #f78da7;--wp--preset--color--vivid-red: #cf2e2e;--wp--preset--color--luminous-vivid-orange: #ff6900;--wp--preset--color--luminous-vivid-amber: #fcb900;--wp--preset--color--light-green-cyan: #7bdcb5;--wp--preset--color--vivid-green-cyan: #00d084;--wp--preset--color--pale-cyan-blue: #8ed1fc;--wp--preset--color--vivid-cyan-blue: #0693e3;--wp--preset--color--vivid-purple: #9b51e0;--wp--preset--color--base: #ffffff;--wp--preset--color--contrast: #000000;--wp--preset--color--primary: #9DFF20;--wp--preset--color--secondary: #345C00;--wp--preset--color--tertiary: #F6F6F6;--wp--preset--gradient--vivid-cyan-blue-to-vivid-purple: linear-gradient(135deg,rgba(6,147,227,1) 0%,rgb(155,81,224) 100%);--wp--preset--gradient--light-green-cyan-to-vivid-green-cyan: linear-gradient(135deg,rgb(122,220,180) 0%,rgb(0,208,130) 100%);--wp--preset--gradient--luminous-vivid-amber-to-luminous-vivid-orange: linear-gradient(135deg,rgba(252,185,0,1) 0%,rgba(255,105,0,1) 100%);--wp--preset--gradient--luminous-vivid-orange-to-vivid-red: linear-gradient(135deg,rgba(255,105,0,1) 0%,rgb(207,46,46) 100%);--wp--preset--gradient--very-light-gray-to-cyan-bluish-gray: linear-gradient(135deg,rgb(238,238,238) 0%,rgb(169,184,195) 100%);--wp--preset--gradient--cool-to-warm-spectrum: linear-gradient(135deg,rgb(74,234,220) 0%,rgb(151,120,209) 20%,rgb(207,42,186) 40%,rgb(238,44,130) 60%,rgb(251,105,98) 80%,rgb(254,248,76) 100%);--wp--preset--gradient--blush-light-purple: linear-gradient(135deg,rgb(255,206,236) 0%,rgb(152,150,240) 100%);--wp--preset--gradient--blush-bordeaux: linear-gradient(135deg,rgb(254,205,165) 0%,rgb(254,45,45) 50%,rgb(107,0,62) 100%);--wp--preset--gradient--luminous-dusk: linear-gradient(135deg,rgb(255,203,112) 0%,rgb(199,81,192) 50%,rgb(65,88,208) 100%);--wp--preset--gradient--pale-ocean: linear-gradient(135deg,rgb(255,245,203) 0%,rgb(182,227,212) 50%,rgb(51,167,181) 100%);--wp--preset--gradient--electric-grass: linear-gradient(135deg,rgb(202,248,128) 0%,rgb(113,206,126) 100%);--wp--preset--gradient--midnight: linear-gradient(135deg,rgb(2,3,129) 0%,rgb(40,116,252) 100%);--wp--preset--font-size--small: clamp(0.875rem, 0.875rem + ((1vw - 0.2rem) * 0.227), 1rem);--wp--preset--font-size--medium: clamp(1rem, 1rem + ((1vw - 0.2rem) * 0.227), 1.125rem);--wp--preset--font-size--large: clamp(1.75rem, 1.75rem + ((1vw - 0.2rem) * 0.227), 1.875rem);--wp--preset--font-size--x-large: 2.25rem;--wp--preset--font-size--xx-large: clamp(4rem, 4rem + ((1vw - 0.2rem) * 10.909), 10rem);--wp--preset--font-family--dm-sans: "DM Sans", sans-serif;--wp--preset--font-family--ibm-plex-mono: 'IBM Plex Mono', monospace;--wp--preset--font-family--inter: "Inter", sans-serif;--wp--preset--font-family--system-font: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen-Sans,Ubuntu,Cantarell,"Helvetica Neue",sans-serif;--wp--preset--font-family--source-serif-pro: "Source Serif Pro", serif;--wp--preset--spacing--20: 0.44rem;--wp--preset--spacing--30: clamp(1.5rem, 5vw, 2rem);--wp--preset--spacing--40: clamp(1.8rem, 1.8rem + ((1vw - 0.48rem) * 2.885), 3rem);--wp--preset--spacing--50: clamp(2.5rem, 8vw, 4.5rem);--wp--preset--spacing--60: clamp(3.75rem, 10vw, 7rem);--wp--preset--spacing--70: clamp(5rem, 5.25rem + ((1vw - 0.48rem) * 9.096), 8rem);--wp--preset--spacing--80: clamp(7rem, 14vw, 11rem);--wp--preset--shadow--natural: 6px 6px 9px rgba(0, 0, 0, 0.2);--wp--preset--shadow--deep: 12px 12px 50px rgba(0, 0, 0, 0.4);--wp--preset--shadow--sharp: 6px 6px 0px rgba(0, 0, 0, 0.2);--wp--preset--shadow--outlined: 6px 6px 0px -3px rgba(255, 255, 255, 1), 6px 6px rgba(0, 0, 0, 1);--wp--preset--shadow--crisp: 6px 6px 0px rgba(0, 0, 0, 1);}:root :where(.is-layout-flow)  :first-child{margin-block-start: 0;}:root :where(.is-layout-flow)  :last-child{margin-block-end: 0;}:root :where(.is-layout-flow)  *{margin-block-start: 1.5rem;margin-block-end: 0;}:root :where(.is-layout-constrained)  :first-child{margin-block-start: 0;}:root :where(.is-layout-constrained)  :last-child{margin-block-end: 0;}:root :where(.is-layout-constrained)  *{margin-block-start: 1.5rem;margin-block-end: 0;}:root :where(.is-layout-flex){gap: 1.5rem;}:root :where(.is-layout-grid){gap: 1.5rem;}body .is-layout-flex{display: flex;}.is-layout-flex{flex-wrap: wrap;align-items: center;}.is-layout-flex  :is(*, div){margin: 0;}body .is-layout-grid{display: grid;}.is-layout-grid  :is(*, div){margin: 0;}.has-black-color{color: var(--wp--preset--color--black) !important;}.has-cyan-bluish-gray-color{color: var(--wp--preset--color--cyan-bluish-gray) !important;}.has-white-color{color: var(--wp--preset--color--white) !important;}.has-pale-pink-color{color: var(--wp--preset--color--pale-pink) !important;}.has-vivid-red-color{color: var(--wp--preset--color--vivid-red) !important;}.has-luminous-vivid-orange-color{color: var(--wp--preset--color--luminous-vivid-orange) !important;}.has-luminous-vivid-amber-color{color: var(--wp--preset--color--luminous-vivid-amber) !important;}.has-light-green-cyan-color{color: var(--wp--preset--color--light-green-cyan) !important;}.has-vivid-green-cyan-color{color: var(--wp--preset--color--vivid-green-cyan) !important;}.has-pale-cyan-blue-color{color: var(--wp--preset--color--pale-cyan-blue) !important;}.has-vivid-cyan-blue-color{color: var(--wp--preset--color--vivid-cyan-blue) !important;}.has-vivid-purple-color{color: var(--wp--preset--color--vivid-purple) !important;}.has-black-background-color{background-color: var(--wp--preset--color--black) !important;}.has-cyan-bluish-gray-background-color{background-color: var(--wp--preset--color--cyan-bluish-gray) !important;}.has-white-background-color{background-color: var(--wp--preset--color--white) !important;}.has-pale-pink-background-color{background-color: var(--wp--preset--color--pale-pink) !important;}.has-vivid-red-background-color{background-color: var(--wp--preset--color--vivid-red) !important;}.has-luminous-vivid-orange-background-color{background-color: var(--wp--preset--color--luminous-vivid-orange) !important;}.has-luminous-vivid-amber-background-color{background-color: var(--wp--preset--color--luminous-vivid-amber) !important;}.has-light-green-cyan-background-color{background-color: var(--wp--preset--color--light-green-cyan) !important;}.has-vivid-green-cyan-background-color{background-color: var(--wp--preset--color--vivid-green-cyan) !important;}.has-pale-cyan-blue-background-color{background-color: var(--wp--preset--color--pale-cyan-blue) !important;}.has-vivid-cyan-blue-background-color{background-color: var(--wp--preset--color--vivid-cyan-blue) !important;}.has-vivid-purple-background-color{background-color: var(--wp--preset--color--vivid-purple) !important;}.has-black-border-color{border-color: var(--wp--preset--color--black) !important;}.has-cyan-bluish-gray-border-color{border-color: var(--wp--preset--color--cyan-bluish-gray) !important;}.has-white-border-color{border-color: var(--wp--preset--color--white) !important;}.has-pale-pink-border-color{border-color: var(--wp--preset--color--pale-pink) !important;}.has-vivid-red-border-color{border-color: var(--wp--preset--color--vivid-red) !important;}.has-luminous-vivid-orange-border-color{border-color: var(--wp--preset--color--luminous-vivid-orange) !important;}.has-luminous-vivid-amber-border-color{border-color: var(--wp--preset--color--luminous-vivid-amber) !important;}.has-light-green-cyan-border-color{border-color: var(--wp--preset--color--light-green-cyan) !important;}.has-vivid-green-cyan-border-color{border-color: var(--wp--preset--color--vivid-green-cyan) !important;}.has-pale-cyan-blue-border-color{border-color: var(--wp--preset--color--pale-cyan-blue) !important;}.has-vivid-cyan-blue-border-color{border-color: var(--wp--preset--color--vivid-cyan-blue) !important;}.has-vivid-purple-border-color{border-color: var(--wp--preset--color--vivid-purple) !important;}.has-vivid-cyan-blue-to-vivid-purple-gradient-background{background: var(--wp--preset--gradient--vivid-cyan-blue-to-vivid-purple) !important;}.has-light-green-cyan-to-vivid-green-cyan-gradient-background{background: var(--wp--preset--gradient--light-green-cyan-to-vivid-green-cyan) !important;}.has-luminous-vivid-amber-to-luminous-vivid-orange-gradient-background{background: var(--wp--preset--gradient--luminous-vivid-amber-to-luminous-vivid-orange) !important;}.has-luminous-vivid-orange-to-vivid-red-gradient-background{background: var(--wp--preset--gradient--luminous-vivid-orange-to-vivid-red) !important;}.has-very-light-gray-to-cyan-bluish-gray-gradient-background{background: var(--wp--preset--gradient--very-light-gray-to-cyan-bluish-gray) !important;}.has-cool-to-warm-spectrum-gradient-background{background: var(--wp--preset--gradient--cool-to-warm-spectrum) !important;}.has-blush-light-purple-gradient-background{background: var(--wp--preset--gradient--blush-light-purple) !important;}.has-blush-bordeaux-gradient-background{background: var(--wp--preset--gradient--blush-bordeaux) !important;}.has-luminous-dusk-gradient-background{background: var(--wp--preset--gradient--luminous-dusk) !important;}.has-pale-ocean-gradient-background{background: var(--wp--preset--gradient--pale-ocean) !important;}.has-electric-grass-gradient-background{background: var(--wp--preset--gradient--electric-grass) !important;}.has-midnight-gradient-background{background: var(--wp--preset--gradient--midnight) !important;}.has-small-font-size{font-size: var(--wp--preset--font-size--small) !important;}.has-medium-font-size{font-size: var(--wp--preset--font-size--medium) !important;}.has-large-font-size{font-size: var(--wp--preset--font-size--large) !important;}.has-x-large-font-size{font-size: var(--wp--preset--font-size--x-large) !important;}:link,
:visited,
:hover,
:active,
:-webkit-any-link {
  text-decoration:none;
	color: inherit;
}

#span-2174-168  p  a {
    color: #00daaa;
    text-decoration: underline !important;
}

div#-slide-menu-456-11  .oxy-slide-menu_inner  #menu-menu-primary-1  .menu-item  .sub-menu  .all_insurance_menu_header {
    border-bottom: 1px solid rgba(241, 239, 227, 0.2);
}
div#-slide-menu-456-11  .oxy-slide-menu_inner  #menu-menu-primary-1  .menu-item  .sub-menu {
    padding-right: 24px;
}
.all_insurance_menu_header {
		border-bottom: 1px solid var(--line-4-grey);
}
.all_insurance_menu_header  a {
		font-weight:bold !important;
		padding-bottom: 16px;
}
li#menu-item-1100  .sub-menu  #menu-item-1742  a {
    padding-top: 16px;
}
li#menu-item-1100  .sub-menu {
    width: 232px;
}




.wp-grid-builder:not(.wpgb-template),.wpgb-facet{opacity:0.01}.wpgb-facet fieldset{margin:0;padding:0;border:none;outline:none;box-shadow:none}.wpgb-facet fieldset:last-child{margin-bottom:40px;}.wpgb-facet fieldset legend.wpgb-sr-only{height:1px;width:1px}


/* Large screens (desktops, 992px and up) */
@media ( min-width: 992px ) {
	.block-visibility-hide-large-screen {
		display: none !important;
	}
}

/* Medium screens (tablets, between 768px and 992px) */
@media ( min-width: 768px ) and ( max-width: 991.98px ) {
	.block-visibility-hide-medium-screen {
		display: none !important;
	}
}

/* Small screens (mobile devices, less than 768px) */
@media ( max-width: 767.98px ) {
	.block-visibility-hide-small-screen {
		display: none !important;
	}
}


.rll-youtube-player{position:relative;padding-bottom:56.23%;height:0;overflow:hidden;max-width:100%;}.rll-youtube-player:focus-within{outline: 2px solid currentColor;outline-offset: 5px;}.rll-youtube-player iframe{position:absolute;top:0;left:0;width:100%;height:100%;z-index:100;background:0 0}.rll-youtube-player img{bottom:0;display:block;left:0;margin:auto;max-width:100%;width:100%;position:absolute;right:0;top:0;border:none;height:auto;-webkit-transition:.4s all;-moz-transition:.4s all;transition:.4s all}.rll-youtube-player img:hover{-webkit-filter:brightness(75%)}.rll-youtube-player .play{height:100%;width:100%;left:0;top:0;position:absolute;background:url(https://www.heygoody.com/th/wp-content/plugins/wp-rocket/assets/img/youtube.png) no-repeat center;background-color: transparent !important;cursor:pointer;border:none;}.wp-embed-responsive .wp-has-aspect-ratio .rll-youtube-player{position:absolute;padding-bottom:0;width:100%;height:100%;top:0;bottom:0;left:0;right:0}







(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':
new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],
j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src=
'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);
})(window,document,'script','dataLayer','GTM-WRLZTM2');




 
!function(e,r){if(void 0!==e&&!e.adbrix){var t={queue:[]},o=navigator.userAgent.toLowerCase(),n=r.createElement("script"),i="Netscape"===navigator.appName&&-1!==navigator.userAgent.search("Trident")||-1!==o.indexOf("msie")?"abx-web-sdk.ie.min.js":"abx-web-sdk.min.js";n.type="text/javascript",n.async=!0,n.src="//static.adbrix.io/web-sdk/latest/"+i,n.onload=function(){e.adbrix.runQueuedFunctions?e.adbrix.runQueuedFunctions():console.log("[adbrix] Error: could not load SDK")};var a=r.getElementsByTagName("script")[0];a.parentNode.insertBefore(n,a);["init","onInitialized","login","getUserId","logout","userProperty.get","userProperty.getAll","userProperty.addOrUpdate","userProperty.remove","userProperty.removes","common.signUp","common.invite","common.useCredit","common.purchase","event.send","debug.traceListener","commerceAttr.categories","commerceAttr.product"].forEach(function(e){var r=e.split("."),o=r.pop();r.reduce(function(e,r){return e[r]=e[r]||{}},t)[o]=function(){t.queue.push([e,arguments])}}),e.adbrix=t}}(window,document);


    window.adbrix.init({
        appkey: "F4VJSEYm80WoKISGHbL2Dg",
        webSecretkey: "CXMQnacOnE6rXb9gJLVXZA",

        //web push 
        push: {
            enable: true,
            serviceWorkerOptions: {
                file_name: "service-worker.js",
                file_path: "/",
                scope: "/",
                //example https://www.heygoody.com/scripts/service-worker.js
            },
        },

        //web pop up
        inWebMessage: {
            enable: true,
            openInNewWindow: true,
            zIndex: 9999,
            fetchListener: function (message) {
                console.log("fetch_listener " + message);
            },
            clickListener: function (actionId, actionType, actionArg, isClosed) {
                console.log(
                "click_listener " + actionId + actionType + actionArg + isClosed
                );
            },
        },

    });







function OptanonWrapper() { }




	document.getElementById("fn-lincense").addEventListener("click",()={
		$("#BrokerLicense").modal("show"); 
	});
	document.getElementById("fn-lincenseOnline").addEventListener("click",()={
		$("#BrokerLicenseOnline").modal("show"); 
	});


	

	.wp-grid-builder .wpgb-card.wpgb-card-hidden .wpgb-card-wrapper{opacity:1!important;visibility:visible!important;transform:none!important}.wpgb-facet {opacity:1!important;pointer-events:auto!important}.wpgb-facet *:not(.wpgb-pagination-facet){display:none}
@font-face{font-family:"DM Sans";font-style:normal;font-weight:400;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/dm-sans/DMSans-Regular.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"DM Sans";font-style:italic;font-weight:400;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/dm-sans/DMSans-Regular-Italic.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"DM Sans";font-style:normal;font-weight:700;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/dm-sans/DMSans-Bold.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"DM Sans";font-style:italic;font-weight:700;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/dm-sans/DMSans-Bold-Italic.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"IBM Plex Mono";font-style:normal;font-weight:300;font-display:block;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/ibm-plex-mono/IBMPlexMono-Light.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"IBM Plex Mono";font-style:normal;font-weight:400;font-display:block;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/ibm-plex-mono/IBMPlexMono-Regular.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"IBM Plex Mono";font-style:italic;font-weight:400;font-display:block;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/ibm-plex-mono/IBMPlexMono-Italic.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"IBM Plex Mono";font-style:normal;font-weight:700;font-display:block;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/ibm-plex-mono/IBMPlexMono-Bold.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:Inter;font-style:normal;font-weight:200 900;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/inter/Inter-VariableFont_slnt,wght.ttf') format('truetype');font-stretch:normal;}
@font-face{font-family:"Source Serif Pro";font-style:normal;font-weight:200 900;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/source-serif-pro/SourceSerif4Variable-Roman.ttf.woff2') format('woff2');font-stretch:normal;}
@font-face{font-family:"Source Serif Pro";font-style:italic;font-weight:200 900;font-display:fallback;src:url('https://www.heygoody.com/th/wp-content/themes/oxygen-is-not-a-theme/assets/fonts/source-serif-pro/SourceSerif4Variable-Italic.ttf.woff2') format('woff2');font-stretch:normal;}






.rll-youtube-player, [data-lazy-src]{display:none !important;}









document.addEventListener("DOMContentLoaded", () = {
    createCookie("gtm-carInfo", createBase64EncodedCookie(getCarInfo()), "gtm-carInfo");
    createCookie("gtm-userData", createBase64EncodedCookie(getUserData()), "gtm-userData");
});
window.onscroll = function() {
	/*
	if(window.scrollY  0){
		$("#onetrust-consent-sdk").fadeIn();
	}else if (window.scrollY == 0){
		$("#onetrust-consent-sdk").fadeOut();
	}
	*/
	if (window.scrollY  0) {
		 //console.log("show");
        $("#onetrust-consent-sdk").addClass("show");
    } else {
        $("#onetrust-consent-sdk").removeClass("show");
    }
 };
function createBase64EncodedCookie(data) {
    return btoa(JSON.stringify(data));
}

function createCookie(cookieName, cookieValue, existingCookieName) {
    if (!getCookie(existingCookieName)) {
        document.cookie = `${cookieName}=${cookieValue}; path=/; samesite=Lax`;
    }
}

function getCarInfo() {
    return {
        car_type: "",
        car_brand: "",
        car_model: "",
        car_submodel: "",
        car_year: "",
        car_last_insurance: "",
        car_current_insurance: "",
        car_coverage_date: "",
        car_registered_province: "",
        birthyear: ""
    };
}

function getUserData() {
    const guestUserID = generateGuestUserID();
    return {
        user_type: "guest",
        user_id: guestUserID,
        guest_id: guestUserID,
        customer_id: "",
        user_first_name: "",
        user_last_name: "",
        user_phone: "",
        user_email: ""
    };
}

function generateGuestUserID() {
    const currentDate = new Date();
    const year = currentDate.getFullYear();
    const month = (currentDate.getMonth() + 1).toString().padStart(2, '0');
    const day = currentDate.getDate().toString().padStart(2, '0');
    const randomPart = (Math.floor(Math.random() * 1000000)).toString().padStart(6, '0');
    return "HGG" + year + month + day + randomPart;
}

function getCookie(name) {
    const nameEQ = name + "=";
    const cookies = document.cookie.split(";");
    for (let i = 0; i 



						ผลิตภัณฑ์ประกันโปรโมชั่น &amp; กิจกรรมเกี่ยวกับเราช่วยเหลือgoodyTalksติดต่อเราเข้าสู่ระบบเข้าสู่ระบบ← กลับประกันรถยนต์

	ประกันรถยนต์ทั้งหมด
	ประกันรถยนต์ไฟฟ้า EV
	ประกันรถยนต์ชั้น 1
	ประกันรถยนต์ชั้น 2+, 2
	ประกันรถยนต์ชั้น 3+, 3


ทำไมต้อง heygoody?
ช่วยเหลือ

	การซื้อประกันและชำระเงิน
	ค้นหาอู่ซ่อม


สาระประกันดี

	บทความ
	โปรโมชั่น
	ข่าวสาร


ติดต่อเรา
เข้าสู่ระบบ
		เข้าสู่ระบบ
		  ประกันรถยนต์

	ประกันรถยนต์ทั้งหมด
	ประกันรถยนต์ไฟฟ้า EV
	ประกันรถยนต์ชั้น 1
	ประกันรถยนต์ชั้น 2+, 2
	ประกันรถยนต์ชั้น 3+, 3


ทำไมต้อง heygoody?
ช่วยเหลือ

	การซื้อประกันและชำระเงิน
	ค้นหาอู่ซ่อม


สาระประกันดี

	บทความ
	โปรโมชั่น
	ข่าวสาร


ติดต่อเรา


		ค้นหา

		
		

		
				window.addEventListener('DOMContentLoaded', function() {
			jQuery(document).ready(function() {
				var selector = "#_header-26-11",
					scrollval = parseInt("52");
				if (!scrollval || scrollval  scrollval 
																) {
								if (
																		!jQuery(selector).hasClass("oxy-sticky-header-active")) {
									if (jQuery(selector).css('position')!='absolute') {
										jQuery("body").css("margin-top", jQuery(selector).outerHeight());
									}
									jQuery(selector)
										.addClass("oxy-sticky-header-active")
																	}
							}
							else {
								jQuery(selector)
									.removeClass("oxy-sticky-header-fade-in")
									.removeClass("oxy-sticky-header-active");
								if (jQuery(selector).css('position')!='absolute') {
									jQuery("body").css("margin-top", "");
								}
							}
							scrollTopOld = jQuery(this).scrollTop();
						}
					})
				}
			});
		});
    document.addEventListener("DOMContentLoaded", async () = {
    await fetchTextSlide(); // wait api 
    initializeTicker(); // เริ่มการตั้งค่า animation
});

async function fetchTextSlide() {
    try {
        const response = await fetch('/th/wp-content/uploads/add-on/textslide/get_text.php');
        const data = await response.json();

        const textSlideContainer = document.getElementById('div_block-287-2395');
        if (!data || data.success === false) {
            textSlideContainer.style.display = 'none';
			document.getElementById('code_block-289-2395').style.display = 'none';
            return;
        }

        textSlideContainer.style.display = 'inline-block';
        textSlideContainer.innerHTML = data.html;
		document.getElementById('code_block-289-2395').style.display = 'block';
		document.getElementById('div_block-292-2395').classList.add("slide-text-section");
		document.getElementById('div_block-287-2395').classList.add("slide-text-section");
    } catch (error) {
        console.error('Error fetching text slide:', error);
        document.getElementById('div_block-287-2395').style.display = 'none';
		document.getElementById('code_block-289-2395').style.display = 'none';
    }
}

function initializeTicker() {
    const ticker = document.querySelector(".textslide");
    const tickerWrap = document.querySelector(".ticker-wrap");

    if (!ticker || !tickerWrap) {
        console.warn('Ticker or ticker-wrap not found.');
        return;
    }

    // คำนวณความกว้าง
    const wrapWidth = tickerWrap.offsetWidth; 
    const tickerWidth = ticker.scrollWidth; 

    if (!wrapWidth || !tickerWidth) {
        console.warn('Invalid ticker dimensions.');
        return;
    }

    const totalDistance = wrapWidth + tickerWidth;
    const speed = 100; // px/s (ปรับตามต้องการ)
    const duration = totalDistance / speed; 

    ticker.style.animationName = "ticker-scroll";
    ticker.style.animationDuration = `${duration}s`;

    // สร้าง keyframes dynamically
    const styleSheet = document.styleSheets[0];
    styleSheet.insertRule(`
        @keyframes ticker-scroll {
            0% {
                transform: translateX(${wrapWidth}px);
            }
            100% {
                transform: translateX(-${tickerWidth}px);
            }
        }
    `, styleSheet.cssRules.length);
}

	
function hideTextSlide() {
	document.getElementById('div_block-287-2395').style.display = 'none'; 
	document.getElementById('div_block-421-8').style.display = 'none';
}
	

	
function AddHeightReadmore(id, hafter, hbefore) {
	let numericHbefore = parseInt(hbefore, 10);
	let addHeight = numericHbefore + 260;
    if ($("#" + id).hasClass("active")) {
       	$("#readMoreSection").css('height', hafter + 'px');
        $("#" + id).removeClass("active");
        $("#" + id + "  svg").css('transform', 'rotate(0deg)');
		dataLayer.push({
		   event: "homepage",
		   event_category: "homepage",
		   event_action: "click",
		   event_label: "narrow"
		 });
    } else {
        if (window.innerWidth  svg").css('transform', 'rotate(180deg)');
		dataLayer.push({
		   event: "homepage",
		   event_category: "homepage",
		   event_action: "click",
		   event_label: "expand_detail"
		 });
    }
}





	
document.addEventListener("DOMContentLoaded", function () {
    async function fetchData(url) {
        try {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            return await response.json();
        } catch (error) {
            console.error(`Error fetching data from ${url}:`, error);
            return { error: true };
        }
    }


    function handleCustomSlideData(data) {
        if (data.error) {
            console.error("Custom Slide API error:", data.error);
            return;
        }

        const html = data.html; // HTML สำหรับ Slideshow
        const delays = data.delays; // Array ของ delays

        const slideshowDiv = document.getElementById("slideshow");

        if (slideshowDiv) {
            slideshowDiv.innerHTML = html; // อัปเดต HTML ของ Slideshow
        }

        new heygoodySlideshow('#slideshow', {
            delays: delays,
        });

    }


    fetchData("/th/wp-content/uploads/add-on/slide/get_customslide.php")
        .then(handleCustomSlideData);

});

function hideCopyText() {
	setTimeout(function() {
		var div = document.getElementById('div_block-256-2395');
		if (div) {
			div.style.display = 'none';
		}
	}, 2000);
}

function copyText(text) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
        // Clipboard API 
        navigator.clipboard.writeText(text).then(() = {
            //alert("Copied to clipboard!");
			document.getElementById("div_block-256-2395").style.display = "flex";
			hideCopyText();
        }).catch(err = {
            console.error("Failed to copy: ", err);
            //alert("Failed to copy text.");
        });
    } else {
        // Fallback สำหรับเบราว์เซอร์ที่ไม่รองรับ Clipboard API
        const textarea = document.createElement("textarea");
        textarea.value = text;
        textarea.style.position = "fixed"; // ป้องกันการเลื่อนหน้าจอ
        textarea.style.opacity = "0"; // ซ่อนไว้จากผู้ใช้
        document.body.appendChild(textarea);
        textarea.focus();
        textarea.select();

        try {
            const successful = document.execCommand("copy");
            if (successful) {
                //alert("Copied to clipboard!");
				document.getElementById("div_block-256-2395").style.display = "flex";
				hideCopyText();
            }
        } catch (err) {
            console.error("Fallback: Failed to copy text", err);
            alert("Failed to copy text.");
        }

        // ลบ textarea ออกจาก DOM หลัง copy เสร็จ
        document.body.removeChild(textarea);
    }
}


    async function fetchCountdown() {
        try {
            const response = await fetch('/th/wp-content/uploads/add-on/countdown/get_countdown.php');
            const data = await response.json();
			const campaignElement = document.getElementById('campaign-section');
            if (data.status === 'in_period') {
                // แสดง HTML ที่ส่งมาจาก API
				campaignElement.style.display = 'flex';
                campaignElement.innerHTML = data.html;
				campaignElement.classList.add("active");
                // ตั้งค่า Countdown Timer
                const endTime = new Date(data.timeLeft + Date.now()).toLocaleString("en-US", {
                    timeZone: "Asia/Bangkok"
                });

                function updateCountdown() {
                    const now = new Date().toLocaleString("en-US", {
                        timeZone: "Asia/Bangkok"
                    });

                    const nowTime = new Date(now).getTime();
                    const endTimeMs = new Date(endTime).getTime();
                    const timeLeft = endTimeMs - nowTime;

                    if (status == 'out_of_period') {
                        document.getElementById('campaign-section').style.display = 'none';
                        clearInterval(countdownInterval);
                        return;
                    }

                    // แปลงเวลาที่เหลือเป็น hh:mm:ss (ไม่ปัดเป็นวัน)
                    const totalHours = Math.floor(timeLeft / (1000 * 60 * 60));
                    const minutes = Math.floor((timeLeft % (1000 * 60 * 60)) / (1000 * 60));
                    const seconds = Math.floor((timeLeft % (1000 * 60)) / 1000);

                    // อัปเดตข้อความ
                    document.getElementById('hgd-time-hh').textContent = String(totalHours).padStart(2, '0');
                    document.getElementById('hgd-time-ii').textContent = String(minutes).padStart(2, '0');
                    document.getElementById('hgd-time-ss').textContent = String(seconds).padStart(2, '0');

                    // ซ่อน "campaign-section" หากเหลือเวลา 00:00:00
                    if (totalHours === 0 && minutes === 0 && seconds === 0) {
                        document.getElementById('campaign-section').style.display = 'none';
                        clearInterval(countdownInterval);
                    }
                }

                const countdownInterval = setInterval(updateCountdown, 1000);
                updateCountdown();
            } else {
                //console.log(data.message);
                document.getElementById('campaign-section').style.display = 'none';
            }
        } catch (error) {
            console.error('Error fetching countdown:', error);
            document.getElementById('campaign-section').style.display = 'none';
        }
    }

    fetchCountdown();

  
    
  

  
    
  
ประกันภัยซื้อง่าย เคลมได้จริง ที่ heygoody

เปรียบเทียบราคาประกัน ซื้อเองได้ ไม่ต้องกรอกข้อมูลเพื่อดูราคา ไร้กังวลโดนโทรขายประกันรถยนต์ประกันรถไฟฟ้า EVต่อประกันรถลูกค้าเดิมประกันรถระยะสั้น30-180 วันประกันเดินทางประกันบ้าน/คอนโดประกันสะสมทรัพย์ประกันบำนาญประกันโรคร้ายแรงประกันโรคมะเร็ง
  

  

  



document.addEventListener('DOMContentLoaded', () = {
  const scrollableElement = document.getElementById('div_block-206-2395');
  const customScrollbarThumb = document.getElementById('custom-scrollbar-thumb');
  const customScrollbarContainer = document.getElementById('custom-scrollbar-container');

  scrollableElement.addEventListener('scroll', () = {
    const maxScrollLeft = scrollableElement.scrollWidth - scrollableElement.clientWidth;
    const scrollLeft = scrollableElement.scrollLeft;
    const scrollPercent = scrollLeft / maxScrollLeft;
    

    const thumbWidth = customScrollbarThumb.offsetWidth; 
	  
    const trackWidth = customScrollbarContainer.offsetWidth;
    const maxThumbScroll = trackWidth - thumbWidth;
    customScrollbarThumb.style.transform = `translateX(${scrollPercent * maxThumbScroll}px)`;
  });
	/*
const scrollableElementtable = document.getElementById('div_block-435-3766');

scrollableElementtable.addEventListener('scroll', () = {
	  const maxScrollLefttable = scrollableElementtable.scrollWidth - scrollableElementtable.clientWidth;
	  const scrollLefttable = scrollableElementtable.scrollLeft;
	  const scrollPercenttable = (scrollLefttable / maxScrollLefttable) * 100;
});
*/
});


คูปองทั้งหมดประกันรถยนต์ประกันรถไฟฟ้า EVประกันเดินทาง
  


	
  
เปลี่ยนการซื้อประกันยากๆ&nbsp;แบบเดิม ให้ง๊ายง่าย

ที่ heygoody.com เช็คเบี้ยฟรี!! เห็นราคาทันที ไม่ขอเบอร์ไม่โทรขาย ซื้อออนไลน์ 24 ชม. โปรดี มีผ่อน 0% การันตี 94% ของลูกค้ารักมากก ซื้อเองง่ายได้ใน 3 นาที!! ซื้อประกันกับ heygoody ดียังไง?ไม่โทรขาย ซื้อออนไลน์ง่ายใน 3 นาที เห็นราคาไหนจ่ายราคานั้น เน้นดูแลลูกค้า ขายแต่ของดี มีส่วนลด เคลมได้จริง แชทได้ 24 ชม.•&nbsp;ซื้อง่าย เคลมได้จริง&nbsp;: เว็บใช้งานง่าย แปลภาษาประกันให้อ่านเข้าใจ เห็นข้อมูลครบ เปรียบเทียบได้เอง ซื้อได้ใน 3 นาที •&nbsp;เน้นดูแลไม่เน้นค่าคอม&nbsp; : มีทีมดูแลลูกค้าเต็มที่ทั้ง 24 ชั่วโมง•&nbsp;ขายแต่ของดี&nbsp; : คัดสรรแผนประกันที่คุ้มค่า จากบริษัทประกันชั้นนำ•&nbsp;เห็นราคาไหนจ่ายราคานั้น&nbsp; : โปร่งใส ไม่โกง ไม่แอบแฝง แสดงราคาชัดเจน •&nbsp;ไม่โทรขาย แชทได้ 24 ชม.&nbsp;  : ไม่โทรขายกวนใจ เพราะเราเคารพความเป็นส่วนตัวของลูกค้า ไม่ต้องกรอกข้อมูลส่วนตัวถ้ายังไม่ซื้อ •&nbsp;มีส่วนลดและโปรโมชั่นดี&nbsp;  : ดูคูปองส่วนลดและโปรโมชั่นแนะนำ ได้ที่เว็บไซต์ และ LINE @heygoody 
  

มีปัญหาทักมาได้ตลอด&nbsp;24 ชม.แอด LINE @heygoodyheygoody มอบความคุ้มครองที่ตอบโจทย์&nbsp;จากหลายบริษัทประกันชั้นนำรีวิวจากเหล่า goodyอ่านประสบการณ์ซื้อประกันจากลูกค้า heygoodyสมาชิกหมายเลข **389**ประกันไม่โทรขายคือประกันออนไลน์ขายในเว็บไม่ผ่านตัวแทนค่ะ ราคาถูกดีด้วย เพิ่งทำผ่านเว็บ heygoody มา มีประกันให้เลือกเยอะ ตั้งแต่ดูจน
ซื้อเสร็จ ไม่มีคนโทรมาซักคน ถูกใจมากจากช่องทาง Pantip.comสมาชิกหมายเลข **932**เคลมผ่านบริษัทประกันได้เลย แต่จะติดต่อผ่าน heygoody ให้ช่วยก็ได้ เท่าที่เคยติดต่อไป 
2-3 หน พนักงานของ heygoody ตอบกลับไว
ครับจากช่องทาง Pantip.comสมาชิกหมายเลข **447**ไปเล่นเว็บมาสองรอบก่อนซื้อ ไม่มีขอข้อมูลเบอร์
แล้วโทรมาตามจี้ ถือว่าซื้อง่าย และไม่โทรตามคำ
เคลม
จากช่องทาง Pantip.comลูกค้า heygoody ไม่ระบุชื่อชอบซื้อประกันที่นี่ค่ะ ทำเองไม่ต้องมีเซลล์โทรมาขาย สบายใจ  แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อสะดวกดีครับ สะดวกดีครับ ไม่รู้สึกรำคาญจากการต้องรับโทรศัพท์แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อแอพดีอยู่แล้วใช้ง่ายชอบตรงที่ไม่โทรจุกจิก และราคาถูกกว่าที่อื่น
แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อนี่เป็นการใช้คนแรกก็รู้สึกประทับใจ บริการดีไม่ว่าจะเป็นพนักงานที่คุยโทรศัพท์ด้วยหรือที่ตอบบน LINE สะดวกรวดเร็ว ทำทำเสร็จภายใน 10 นาทีก็ได้กรมธรรม์แล้วค่ะ  แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อดีค่ะ เคยใช้บริการ 1 ครั้ง กรอกข้อมูลง่ายได้เอกสารไว ขอให้เพิ่มพันธมิตรบริษัทประกันเยอะๆ จะได้มีตัวเลือกหลากหลายมากที่สุดแบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อบริการดีมากค่ะ ต่อประกันได้สะดวกแถมมีของแถมทุกรอบ
แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อบริการดีมาก ๆ มีการแจ้งเตือนตลอด มีโปรโมชั่นดี ๆ เสมอ  แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อประทับใจระบบมากๆครับ แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อตอบแชทไวมากค่ะ ดีสุดๆ
แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อพนักงานตอบคำถามที่สงสัยได้ไว  แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อบริการดี รวดเร็วค่ะ แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่ออยากให้รักษาคุณภาพไว้แบบนี้ตลอดไปครับผม
แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อบริการดี มีปัญหาทักไปตอบทันที  แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อดีแล้วค่ะใส่ใจลูกค้าดี แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อแอดมินน่ารัก บริการดี ตอบไว ให้คำตอบชัดเจน น่ารักมากๆ ขอบคุณค่ะ
แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อดีอยู่แล้วค่ะ บริการดีแก้ปัญหาได้ตรงจุด  แบบสอบถามหลังได้รับบริการลูกค้า heygoody ไม่ระบุชื่อไม่มีค่ะ บริการดี โดยเฉพาะแอดมินตอบเคลียร์ทุกคำถามและรวดเร็วค่ะ แบบสอบถามหลังได้รับบริการ&nbsp;
  

  

const slider = document.getElementById('slider-review');
const prevBtn = document.getElementById('prevBtn-review');
const nextBtn = document.getElementById('nextBtn-review');
const slides = document.querySelectorAll('.slide-section');
const dotsContainer = document.getElementById('dots-review');
const containerWidth = document.querySelector('.slider-container-review').offsetWidth;

let currentIndex = 0; // Initialize current slide index
let startX = 0;
let currentTranslate = 0;
let isDragging = false;

// Determine slideWidth based on screen size
let slideWidth = calculateSlideWidth();

// Calculate initial offset and set prevTranslate
let offset = window.innerWidth  {
  slideWidth = calculateSlideWidth();
  moveSlider(); // Adjust slider position based on the new slideWidth
});

// Create dots
slides.forEach((_, index) = {
  const dot = document.createElement('div');
  dot.classList.add('dot');
  if (index === 0) dot.classList.add('active');
  dotsContainer.appendChild(dot);
});
const dots = document.querySelectorAll('.dot');

function updateDots() {
  dots.forEach((dot, index) = {
    if (index === currentIndex) {
      dot.classList.add('active');
    } else {
      dot.classList.remove('active');
    }
  });
}

function updateButtons() {
  // Update the previous button
  const prevSvgPath = prevBtn.querySelector('path');
  if (currentIndex === 0) {
    prevSvgPath.setAttribute('fill', '#B8B8B8');
    prevBtn.classList.add('disabled'); // Add 'disabled' class to prevBtn
  } else {
    prevSvgPath.setAttribute('fill', '#F9F9F4');
    prevBtn.classList.remove('disabled'); // Remove 'disabled' class from prevBtn
  }

  // Update the next button
  const nextSvgPath = nextBtn.querySelector('path');
  const maxVisibleIndex = Math.floor(containerWidth / slideWidth) + currentIndex - 1;
  if (maxVisibleIndex = slides.length - 1) {
    nextSvgPath.setAttribute('fill', '#B8B8B8');
    nextBtn.classList.add('disabled'); // Add 'disabled' class to nextBtn
  } else {
    nextSvgPath.setAttribute('fill', '#F9F9F4');
    nextBtn.classList.remove('disabled'); // Remove 'disabled' class from nextBtn
  }
}


function moveSlider() {
  let offset = 0;

  // Adjust for tablet and mobile only if not on the first slide
  if (window.innerWidth  0) {
    offset = (containerWidth - slideWidth) / 2; // Center slide
  }

  // Calculate translateX
  const translateX = -currentIndex * slideWidth + offset;
  slider.style.transform = `translateX(${translateX}px)`;
  
  // Update button states and dots
  updateButtons();
  updateDots();
}



prevBtn.addEventListener('click', () = {
  if (prevBtn.classList.contains('disabled')) return; // Prevent click if disabled
  if (currentIndex  0) {
    currentIndex--;
    moveSlider();
  }
});

nextBtn.addEventListener('click', () = {
  if (nextBtn.classList.contains('disabled')) return; // Prevent click if disabled
  if (currentIndex  100 && currentIndex  0) currentIndex--;

  let offset = 0;
  if (window.innerWidth  0) {
    offset = (containerWidth - slideWidth) / 2; // Center slide for other slides
  }

  prevTranslate = -currentIndex * slideWidth + offset;
  moveSlider();
  slider.style.transition = 'transform 0.3s ease-in-out';
}



// Initialize button states and dots
updateButtons();
updateDots();

prevBtn.addEventListener('mousedown', (e) = e.preventDefault());
nextBtn.addEventListener('mousedown', (e) = e.preventDefault());


        const sliderWrapperShelf = document.getElementById('sliderWrapperShelf');
        const slidesShelf = document.querySelectorAll('.product-shelf-section');
        const prevButtonShelf = document.getElementById('prev-shelf');
        const nextButtonShelf = document.getElementById('next-shelf');
        const totalSlidesShelf = slidesShelf.length;

        let currentIndexShelf = 0;

        function updateSliderPositionShelf() {
            const offset = -currentIndexShelf * 386;
            sliderWrapperShelf.style.transform = `translateX(${offset}px)`;
            
            // Update button states
            if (currentIndexShelf === 0) {
                prevButtonShelf.classList.add('shelf-deactive');
                prevButtonShelf.classList.remove('shelf-active');
            } else {
                prevButtonShelf.classList.add('shelf-active');
                prevButtonShelf.classList.remove('shelf-deactive');
            }

            if (currentIndexShelf === totalSlidesShelf - 1) {
                nextButtonShelf.classList.add('shelf-deactive');
                nextButtonShelf.classList.remove('shelf-active');
            } else {
                nextButtonShelf.classList.add('shelf-active');
                nextButtonShelf.classList.remove('shelf-deactive');
            }
        }

        prevButtonShelf.addEventListener('click', () = {
            if (currentIndexShelf  0) {
                currentIndexShelf -= 1;
                updateSliderPositionShelf();
				dataLayer.push({ event: "homepage", event_category: "homepage", event_action: "click", event_label: "shelf_prev_button", });
            }
        });

        nextButtonShelf.addEventListener('click', () = {
            if (currentIndexShelf 

  let couponData = {};
  let dynamicCategoryTracking;


  function copyCode(code) {
    if (!code) {
      console.error('ไม่พบโค้ดที่ต้องการคัดลอก');
      return;
    }

    const buttons = document.querySelectorAll('.btn-orange');
    buttons.forEach(btn = (btn.disabled = true));

    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard
        .writeText(code)
        .then(() = {
          showPopupAndEnableButtons();
        })
        .catch(err = {
          console.error('การคัดลอกล้มเหลว:', err);
          fallbackCopyTextToClipboard(code);
        });
    } else {
      fallbackCopyTextToClipboard(code);
    }
  }

  function fallbackCopyTextToClipboard(text) {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    textArea.style.position = 'fixed';
    textArea.style.top = '0';
    textArea.style.left = '0';
    textArea.style.width = '2em';
    textArea.style.height = '2em';
    textArea.style.padding = '0';
    textArea.style.border = 'none';
    textArea.style.outline = 'none';
    textArea.style.boxShadow = 'none';
    textArea.style.background = 'transparent';

    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();

    try {
      document.execCommand('copy');
      showPopupAndEnableButtons();
    } catch (err) {
      console.error('Fallback: การคัดลอกล้มเหลว', err);
    }

    document.body.removeChild(textArea);
  }

  function showPopupAndEnableButtons() {
    const popup = document.getElementById('popupCopyCode');
    if (popup) {
      popup.style.display = 'flex';
      setTimeout(() = {
        popup.style.display = 'none';
        const buttons = document.querySelectorAll('.btn-orange');
        buttons.forEach(btn = (btn.disabled = false));
      }, 1500);
    }
  }


  function openCondition(couponId) {
    let foundCoupon = null;

    for (const category in couponData) {
      const coupon = couponData[category].find(cpn = cpn.id === couponId);
      if (coupon) {
        foundCoupon = coupon;
        break;
      }
    }

    if (!foundCoupon) {
      console.error('ไม่พบคูปองที่มี ID: ' + couponId);
      return;
    }

    const condition = foundCoupon.condition?.[0];
    if (!condition) {
      console.error('ไม่พบข้อมูล condition ของคูปอง ID: ' + couponId);
      return;
    }


    const conditionCodeDiv = document.getElementById('condition_code');
    conditionCodeDiv.innerHTML = condition.condition_code || '';


    const dynamicCode = foundCoupon.discount_code;
    if (dynamicCode) {
	  conditionCodeDiv.setAttribute('onclick', `copyCode('${dynamicCode}'); dataLayer.push({ event: "homepage", event_category: "homepage", event_action: "click", event_label: "copy_coupon", additional: "${dynamicCode}", section: "condition_detail" });`);
	  
      conditionCodeDiv.style.cursor = 'pointer';
    } else {

      conditionCodeDiv.removeAttribute('onclick');
      conditionCodeDiv.style.cursor = 'default';
    }

    document.getElementById('condition_discount_code').innerHTML = condition.condition_discount_code || 'ไม่มีข้อมูล';
    document.getElementById('condition_expire_date').innerHTML = condition.condition_expire_date || 'ไม่มีข้อมูล';
    document.getElementById('condition_payment').innerHTML = condition.condition_payment || 'ไม่มีข้อมูล';
    document.getElementById('condition_details').innerHTML = condition.condition_details || 'ไม่มีข้อมูล';


    $('#myCondition').modal('show');
    document.getElementById('myCondition').style.display = 'flex';
  }


  function getAllCoupons() {
    let allCoupons = [];
    for (const catKey in couponData) {
      couponData[catKey].forEach(cpn = {
        const couponCategories = cpn.categories || [catKey];
        allCoupons.push({
          ...cpn,
          categories: couponCategories
        });
      });
    }
    return allCoupons;
  }


  function showCoupons(coupons) {
    const container = document.getElementById('coupon-list');
    container.innerHTML = '';

    if (!Array.isArray(coupons)) {
      console.error('Coupons is not an array');
      return;
    }


    const groups = {};
    coupons.forEach(coupon = {
      let primaryCat = 'uncategorized';
      if (coupon.categories && Array.isArray(coupon.categories) && coupon.categories.length  0) {
        primaryCat = coupon.categories[0];
      } else if (coupon.category) {
        primaryCat = coupon.category;
      }
      if (!groups[primaryCat]) {
        groups[primaryCat] = [];
      }
      groups[primaryCat].push(coupon);
    });


    Object.keys(groups).forEach(group = {
      groups[group].forEach(coupon = {

        const couponContainer = document.createElement('div');
        couponContainer.classList.add('coupon-container');

 
        const couponLeft = document.createElement('div');
        couponLeft.classList.add('coupon-left');
        if (coupon.categories && Array.isArray(coupon.categories)) {
          coupon.categories.forEach(cat = couponLeft.classList.add(cat));
        } else if (coupon.category) {
          couponLeft.classList.add(coupon.category);
        }

        if (coupon.image) {
          const img = document.createElement('img');
          img.src = coupon.image;
          img.alt = coupon.title || '';
          couponLeft.appendChild(img);
        }

        const titleEl = document.createElement('h3');
        titleEl.classList.add('title');
        titleEl.innerHTML = coupon.title;
        couponLeft.appendChild(titleEl);

        const couponRight = document.createElement('div');
        couponRight.classList.add('coupon-right');

        if (coupon.discount_text) {
          const discountTextEl = document.createElement('div');
          discountTextEl.classList.add('discount_text');
          discountTextEl.innerHTML = coupon.discount_text;
          couponRight.appendChild(discountTextEl);
        }
        if (coupon.minimum_text) {
          const minTextEl = document.createElement('div');
          minTextEl.classList.add('minimum_text');
          minTextEl.innerHTML = coupon.minimum_text;
          couponRight.appendChild(minTextEl);
        }
        if (coupon.payment_text) {
          const paymentTextEl = document.createElement('div');
          paymentTextEl.classList.add('payment_text');
          paymentTextEl.innerHTML = coupon.payment_text;
          couponRight.appendChild(paymentTextEl);
        }

        const couponActions = document.createElement('div');
        couponActions.classList.add('coupon-actions');

        const conditionBlock = document.createElement('div');
        const conditionSpan = document.createElement('span');
        conditionSpan.classList.add('condition_text');
        conditionSpan.innerHTML = coupon.condition_text || 'เงื่อนไข';
		conditionSpan.setAttribute('onclick', `openCondition('${coupon.id}'); dataLayer.push({ event: "homepage", event_category: "homepage", event_action: "click", event_label: "condition", additional: "${coupon.discount_code}", section: dynamicCategoryTracking });`);

        const codeBlock = document.createElement('div');
        codeBlock.style.display = 'flex';
        codeBlock.style.alignItems = 'center';
        codeBlock.style.justifyContent = 'space-between';
        codeBlock.style.columnGap = '6px';

        const discountCodeSpan = document.createElement('span');
        discountCodeSpan.classList.add('discount_code');
        discountCodeSpan.innerHTML = coupon.discount_code || '';

        const copyIcon = document.createElement('img');
        copyIcon.src = '/th/wp-content/uploads/2025/01/copy-button-icon_homepage_heygoody.webp';
        copyIcon.width = 16;
        copyIcon.height = 16;

        if (coupon.discount_code) {
          const couponSection = document.createElement('div');
          couponSection.classList.add('coupon-section');
          couponSection.appendChild(copyIcon);
          couponSection.appendChild(discountCodeSpan);
          codeBlock.appendChild(couponSection);

          const divCoverButton = document.createElement('div');
          const copyButton = document.createElement('button');
          copyButton.classList.add('btn-orange');
          copyButton.innerText = 'คัดลอก';
		  copyButton.setAttribute( 'onclick', `copyCode('${coupon.discount_code}'); dataLayer.push({ event: "homepage", event_category: "homepage", event_action: "click", event_label: "copy_coupon", additional: "${coupon.discount_code}", section: dynamicCategoryTracking });` );

          conditionBlock.style.marginBottom = '4px';
          divCoverButton.appendChild(copyButton);
          codeBlock.appendChild(divCoverButton);
        }

        conditionBlock.appendChild(conditionSpan);
        couponActions.appendChild(conditionBlock);
        couponActions.appendChild(codeBlock);
        couponRight.appendChild(couponActions);

        // ประกอบร่างทั้งหมด
        couponContainer.appendChild(couponLeft);
        couponContainer.appendChild(couponRight);
        container.appendChild(couponContainer);
      });
    });
  }


  function filterCoupons(categoryKey, element) {

    switch (categoryKey) {
      case 'all':
        dynamicCategoryTracking = 'all';
        break;
      case 'car_insurance':
        dynamicCategoryTracking = 'motor';
        break;
      case 'car_insurance_ev':
        dynamicCategoryTracking = 'ev';
        break;
      case 'travel_insurance':
        dynamicCategoryTracking = 'travel';
        break;
    }

    const allBtns = document.querySelectorAll('.filter_coupon');
    allBtns.forEach(btn = btn.classList.remove('active-filter'));
    if (element) {
      element.classList.add('active-filter');
    }

    let allCoupons = [];
    for (const catKey in couponData) {
      couponData[catKey].forEach(cpn = {
        const couponCategories = cpn.categories || [catKey];
        allCoupons.push({ ...cpn, categories: couponCategories });
      });
    }

    let coupons = [];
    if (categoryKey === 'all') {
      coupons = allCoupons;
    } else {
      coupons = allCoupons.filter(cpn = cpn.categories.includes(categoryKey));
    }

    coupons.sort((a, b) = a.sort_order - b.sort_order);

    if (coupons.length === 0) {
      document.getElementById('coupon-list').innerHTML = 'ไม่พบคูปองในหมวดหมู่ที่เลือก

';
      prevBtnCoupon.style.display = 'none';
      nextBtnCoupon.style.display = 'none';
      return;
    } else {
      prevBtnCoupon.style.display = '';
      nextBtnCoupon.style.display = '';
    }

    showCoupons(coupons);

 
    currentIndexCoupon = 0;
    totalCoupons = coupons.length;


    if (totalCoupons  0) {
      currentIndexCoupon--;
      updateSliderCoupon();
    }
  }
	const imgCopy = "";

  document.addEventListener('DOMContentLoaded', () = {

    fetch('/th/wp-content/uploads/add-on/promotioncoupon/couponData.json') 
      .then(response = response.json())
      .then(data = {
		
        couponData = data;

        const defaultButton = document.querySelector('.filter_coupon[onclick*="filterCoupons(\'all\'"]');
        if (defaultButton) {
          defaultButton.classList.add('active-filter');
        }

        filterCoupons('all', defaultButton);
      })
      .catch(err = {
        console.error('Failed to load coupon data:', err);
      });
  });

คัดลอกแล้ว
  
เงื่อนไขเพิ่มเติมโค้ด: NULLส่วนลด: NULLหมดอายุ NULLการชำระเงินNULLเงื่อนไขการใช้งานNULL
document.addEventListener('DOMContentLoaded', () = {
    if (window.innerWidth = 1199) {
        setTimeout(() = {
            const mouseEvent = new MouseEvent('mousemove', {
                bubbles: true,
                cancelable: true,
                view: window,
            });

            document.dispatchEvent(mouseEvent);
        }, 1000);
    }
	setTimeout(() = {
	var slides = document.querySelectorAll("#slideshow a.slide");

	  slides.forEach(function(slide, index) {

		var campaignId = index + 1;
		var campaignName = slide.href.replace("https://www.heygoody.com/th", "");

		slide.addEventListener("click", function(event) {

		  dataLayer.push({
			event: "dynamic_banner",
			campaign_id: campaignId.toString(),
			campaign_name: campaignName,
			creative_slot: "homepage"
		  });

		});
	  });
		
	      var slideshowfortracking = document.getElementById("slideshow");
  
		  var divElementstracking = Array.from(slideshowfortracking.children).filter(function(child) {
			return child.tagName.toLowerCase() === 'div';
		  });


		  if (divElementstracking.length = 2) {

			var firstDiv = divElementstracking[0];
			firstDiv.addEventListener("click", function() {
			  dataLayer.push({
				event: "homepage",
				event_category: "homepage",
				event_action: "click",
				event_label: "herobanner_left_button"
			  });
			});


			var secondDiv = divElementstracking[1];
			secondDiv.addEventListener("click", function() {
			  dataLayer.push({
				event: "homepage",
				event_category: "homepage",
				event_action: "click",
				event_label: "herobanner_right_button"
			  });
			});
		  }
	}, 500);
});

16 รางวัลการันตีความสำเร็จจากเวทีระดับโลกดูรางวัลทั้งหมด
  
Insure Tech Connect Asia Brokerage Breakthrough · Data 
Analytics Master Awards - 2024 Global Retail Banking Innovation Best Customer Centric Business Model - 2024 New York Festivals Awards 2024 Best Customer Centric Business Model - 2024 The Work 2024 Film/TV Craft · Film/Web Film · Culture · Work for Good · Branded Content+Entertainment - 2024 Thailand Influencer Awards 2024 by Tellscore Best Financial &amp; Investment Influencer Campaign - 2024 AdPeople Awards &amp; Symposium 2024 •Silver หมวดหมู่ Craft 
•Bronze หมวดหมู่ Craft 
•Bronze หมวดหมู่ Film Marketing Award of Thailand 2024 Silver -Brand Experience &amp; Communication 
  
ช่องทางการติดต่อเลขที่ใบอนุญาตประกันวินาศภัย ว00015/2556เลขที่ใบอนุญาตเสนอขายประกันภัยผ่านช่องทางอิเล็กทรอนิกส์ อลว 015521000/2563&nbsp;บริษัท เงินติดล้อ จำกัด (มหาชน)ประกันรถยนต์ประกันรถยนต์ทั้งหมดประกันรถยนต์ระยะสั้นประกันรถยนต์ไฟฟ้า EVประกันรถตู้ส่วนบุคคลประกันรถยนต์ชั้น 1ประกันรถยนต์ชั้น 2+, 2ประกันรถยนต์ชั้น 3+, 3ประกันเดินทางประกันเดินทางต่างประเทศประกันสุขภาพประกันโรคร้ายแรงประกันโรคมะเร็งประกันชีวิต (ลดหย่อนภาษี)คำนวนและเปรียบเทียบประกัน
ลดหย่อนภาษีประกันสะสมทรัพย์ประกันบำนาญประกันอื่นๆประกันบ้าน/คอนโดเกี่ยวกับเรารู้จักเราทำไมต้อง heygoody?รางวัลความสำเร็จศูนย์ช่วยเหลือความช่วยเหลือทั้งหมดการใช้งานโปรโมชั่นค้นหาอู่ซ่อมคำถามที่พบบ่อยอื่นๆโปรโมชั่นบทความข่าวสารติดต่อเรา© 2567 บริษัท เงินติดล้อ จำกัด (มหาชน)นโยบายความเป็นส่วนตัว|นโยบายส่วนบุคคลเกี่ยวกับคุกกี้เลขที่ใบอนุญาตประกันวินาศภัย	







    (function(h,o,t,j,a,r){
        h.hj=h.hj||function(){(h.hj.q=h.hj.q||[]).push(arguments)};
        h._hjSettings={hjid:3132820,hjsv:6};
        a=o.getElementsByTagName('head')[0];
        r=o.createElement('script');r.async=1;
        r.src=t+h._hjSettings.hjid+j+h._hjSettings.hjsv;
        a.appendChild(r);
    })(window,document,'https://static.hotjar.com/c/hotjar-','.js?sv=');

chevron-down    
		setTimeout(() = {
    ;window.NREUM||(NREUM={});NREUM.init={distributed_tracing:{enabled:true},privacy:{cookies_enabled:true},ajax:{deny_list:["bam.nr-data.net"]}};

;NREUM.loader_config={accountID:"6392919",trustKey:"6392919",agentID:"1134553161",licenseKey:"NRJS-450b5396455f75c0f1a",applicationID:"1002484837"};
;NREUM.info={beacon:"bam.nr-data.net",errorBeacon:"bam.nr-data.net",licenseKey:"NRJS-450b5396455f75c0f1a",applicationID:"1002484837",sa:1};
;/*! For license information please see nr-loader-spa-1.283.2.min.js.LICENSE.txt */
(()={var e,t,r={8122:(e,t,r)={"use strict";r.d(t,{a:()=i});var n=r(944);function i(e,t){try{if(!e||"object"!=typeof e)return(0,n.R)(3);if(!t||"object"!=typeof t)return(0,n.R)(4);const r=Object.create(Object.getPrototypeOf(t),Object.getOwnPropertyDescriptors(t)),o=0===Object.keys(r).length?e:r;for(let a in o)if(void 0!==e[a])try{if(null===e[a]){r[a]=null;continue}Array.isArray(e[a])&&Array.isArray(t[a])?r[a]=Array.from(new Set([...e[a],...t[a]])):"object"==typeof e[a]&&"object"==typeof t[a]?r[a]=i(e[a],t[a]):r[a]=e[a]}catch(e){(0,n.R)(1,e)}return r}catch(e){(0,n.R)(2,e)}}},2555:(e,t,r)={"use strict";r.d(t,{Vp:()=c,fn:()=s,x1:()=u});var n=r(384),i=r(8122);const o={beacon:n.NT.beacon,errorBeacon:n.NT.errorBeacon,licenseKey:void 0,applicationID:void 0,sa:void 0,queueTime:void 0,applicationTime:void 0,ttGuid:void 0,user:void 0,account:void 0,product:void 0,extra:void 0,jsAttributes:{},userAttributes:void 0,atts:void 0,transactionName:void 0,tNamePlain:void 0},a={};function s(e){try{const t=c(e);return!!t.licenseKey&&!!t.errorBeacon&&!!t.applicationID}catch(e){return!1}}function c(e){if(!e)throw new Error("All info objects require an agent identifier!");if(!a[e])throw new Error("Info for ".concat(e," was never set"));return a[e]}function u(e,t){if(!e)throw new Error("All info objects require an agent identifier!");a[e]=(0,i.a)(t,o);const r=(0,n.nY)(e);r&&(r.info=a[e])}},9417:(e,t,r)={"use strict";r.d(t,{D0:()=h,gD:()=g,xN:()=p});var n=r(3333);const i=e={if(!e||"string"!=typeof e)return!1;try{document.createDocumentFragment().querySelector(e)}catch{return!1}return!0};var o=r(2614),a=r(944),s=r(384),c=r(8122);const u="[data-nr-mask]",d=()={const e={feature_flags:[],experimental:{marks:!1,measures:!1,resources:!1},mask_selector:"*",block_selector:"[data-nr-block]",mask_input_options:{color:!1,date:!1,"datetime-local":!1,email:!1,month:!1,number:!1,range:!1,search:!1,tel:!1,text:!1,time:!1,url:!1,week:!1,textarea:!1,select:!1,password:!0}};return{ajax:{deny_list:void 0,block_internal:!0,enabled:!0,autoStart:!0},distributed_tracing:{enabled:void 0,exclude_newrelic_header:void 0,cors_use_newrelic_header:void 0,cors_use_tracecontext_headers:void 0,allowed_origins:void 0},get feature_flags(){return e.feature_flags},set feature_flags(t){e.feature_flags=t},generic_events:{enabled:!0,autoStart:!0},harvest:{interval:30},jserrors:{enabled:!0,autoStart:!0},logging:{enabled:!0,autoStart:!0},metrics:{enabled:!0,autoStart:!0},obfuscate:void 0,page_action:{enabled:!0},page_view_event:{enabled:!0,autoStart:!0},page_view_timing:{enabled:!0,autoStart:!0},performance:{get capture_marks(){return e.feature_flags.includes(n.$v.MARKS)||e.experimental.marks},set capture_marks(t){e.experimental.marks=t},get capture_measures(){return e.feature_flags.includes(n.$v.MEASURES)||e.experimental.measures},set capture_measures(t){e.experimental.measures=t},capture_detail:!0,resources:{get enabled(){return e.feature_flags.includes(n.$v.RESOURCES)||e.experimental.resources},set enabled(t){e.experimental.resources=t},asset_types:[],first_party_domains:[],ignore_newrelic:!0}},privacy:{cookies_enabled:!0},proxy:{assets:void 0,beacon:void 0},session:{expiresMs:o.wk,inactiveMs:o.BB},session_replay:{autoStart:!0,enabled:!1,preload:!1,sampling_rate:10,error_sampling_rate:100,collect_fonts:!1,inline_images:!1,fix_stylesheets:!0,mask_all_inputs:!0,get mask_text_selector(){return e.mask_selector},set mask_text_selector(t){i(t)?e.mask_selector="".concat(t,",").concat(u):""===t||null===t?e.mask_selector=u:(0,a.R)(5,t)},get block_class(){return"nr-block"},get ignore_class(){return"nr-ignore"},get mask_text_class(){return"nr-mask"},get block_selector(){return e.block_selector},set block_selector(t){i(t)?e.block_selector+=",".concat(t):""!==t&&(0,a.R)(6,t)},get mask_input_options(){return e.mask_input_options},set mask_input_options(t){t&&"object"==typeof t?e.mask_input_options={...t,password:!0}:(0,a.R)(7,t)}},session_trace:{enabled:!0,autoStart:!0},soft_navigations:{enabled:!0,autoStart:!0},spa:{enabled:!0,autoStart:!0},ssl:void 0,user_actions:{enabled:!0,elementAttributes:["id","className","tagName","type"]}}},l={},f="All configuration objects require an agent identifier!";function h(e){if(!e)throw new Error(f);if(!l[e])throw new Error("Configuration for ".concat(e," was never set"));return l[e]}function p(e,t){if(!e)throw new Error(f);l[e]=(0,c.a)(t,d());const r=(0,s.nY)(e);r&&(r.init=l[e])}function g(e,t){if(!e)throw new Error(f);var r=h(e);if(r){for(var n=t.split("."),i=0;i{"use strict";r.d(t,{a:()=c,o:()=s});var n=r(384),i=r(8122);const o={accountID:void 0,trustKey:void 0,agentID:void 0,licenseKey:void 0,applicationID:void 0,xpid:void 0},a={};function s(e){if(!e)throw new Error("All loader-config objects require an agent identifier!");if(!a[e])throw new Error("LoaderConfig for ".concat(e," was never set"));return a[e]}function c(e,t){if(!e)throw new Error("All loader-config objects require an agent identifier!");a[e]=(0,i.a)(t,o);const r=(0,n.nY)(e);r&&(r.loader_config=a[e])}},3371:(e,t,r)={"use strict";r.d(t,{V:()=f,f:()=l});var n=r(8122),i=r(384),o=r(6154),a=r(9324);let s=0;const c={buildEnv:a.F3,distMethod:a.Xs,version:a.xv,originTime:o.WN},u={customTransaction:void 0,disabled:!1,isolatedBacklog:!1,loaderType:void 0,maxBytes:3e4,onerror:void 0,ptid:void 0,releaseIds:{},appMetadata:{},session:void 0,denyList:void 0,timeKeeper:void 0,obfuscator:void 0,harvester:void 0},d={};function l(e){if(!e)throw new Error("All runtime objects require an agent identifier!");if(!d[e])throw new Error("Runtime for ".concat(e," was never set"));return d[e]}function f(e,t){if(!e)throw new Error("All runtime objects require an agent identifier!");d[e]={...(0,n.a)(t,u),...c},Object.hasOwnProperty.call(d[e],"harvestCount")||Object.defineProperty(d[e],"harvestCount",{get:()=++s});const r=(0,i.nY)(e);r&&(r.runtime=d[e])}},9324:(e,t,r)={"use strict";r.d(t,{F3:()=i,Xs:()=o,Yq:()=a,xv:()=n});const n="1.283.2",i="PROD",o="CDN",a="^2.0.0-alpha.17"},6154:(e,t,r)={"use strict";r.d(t,{A4:()=s,OF:()=d,RI:()=i,WN:()=h,bv:()=o,gm:()=a,lR:()=f,m:()=u,mw:()=c,sb:()=l});var n=r(1863);const i="undefined"!=typeof window&&!!window.document,o="undefined"!=typeof WorkerGlobalScope&&("undefined"!=typeof self&&self instanceof WorkerGlobalScope&&self.navigator instanceof WorkerNavigator||"undefined"!=typeof globalThis&&globalThis instanceof WorkerGlobalScope&&globalThis.navigator instanceof WorkerNavigator),a=i?window:"undefined"!=typeof WorkerGlobalScope&&("undefined"!=typeof self&&self instanceof WorkerGlobalScope&&self||"undefined"!=typeof globalThis&&globalThis instanceof WorkerGlobalScope&&globalThis),s="complete"===a?.document?.readyState,c=Boolean("hidden"===a?.document?.visibilityState),u=""+a?.location,d=/iPad|iPhone|iPod/.test(a.navigator?.userAgent),l=d&&"undefined"==typeof SharedWorker,f=(()={const e=a.navigator?.userAgent?.match(/Firefox[/\s](\d+\.\d+)/);return Array.isArray(e)&&e.length=2?+e[1]:0})(),h=Date.now()-(0,n.t)()},7295:(e,t,r)={"use strict";r.d(t,{Xv:()=a,gX:()=i,iW:()=o});var n=[];function i(e){if(!e||o(e))return!1;if(0===n.length)return!0;for(var t=0;t0?(o=r.substring(0,i),a=r.substring(i)):(o=r,a="");let[s]=o.split(":");n.push({hostname:s,pathname:a})}}function s(e,t){return!(e.lengtht.length)&&t.indexOf(e)===t.length-e.length}function c(e,t){return 0===e.indexOf("/")&&(e=e.substring(1)),0===t.indexOf("/")&&(t=t.substring(1)),""===e||e===t}},1687:(e,t,r)={"use strict";r.d(t,{Ak:()=c,Ze:()=l,x3:()=u});var n=r(7836),i=r(3606),o=r(860),a=r(2646);const s={};function c(e,t){const r={staged:!1,priority:o.P3[t]||0};d(e),s[e].get(t)||s[e].set(t,r)}function u(e,t){e&&s[e]&&(s[e].get(t)&&s[e].delete(t),h(e,t,!1),s[e].size&&f(e))}function d(e){if(!e)throw new Error("agentIdentifier required");s[e]||(s[e]=new Map)}function l(e="",t="feature",r=!1){if(d(e),!e||!s[e].get(t)||r)return h(e,t);s[e].get(t).staged=!0,f(e)}function f(e){const t=Array.from(s[e]);t.every((([e,t])=t.staged))&&(t.sort(((e,t)=e[1].priority-t[1].priority)),t.forEach((([t])={s[e].delete(t),h(e,t)})))}function h(e,t,r=!0){const o=e?n.ee.get(e):n.ee,s=i.i.handlers;if(!o.aborted&&o.backlog&&s){if(r){const e=o.backlog[t],r=s[t];if(r){for(let t=0;e&&t{Object.values(t||{}).forEach((t={t[0]?.on&&t[0]?.context()instanceof a.y&&t[0].on(e,t[1])}))}))}}o.isolatedBacklog||delete s[t],o.backlog[t]=null,o.emit("drain-"+t,[])}}function p(e,t){var r=e[1];Object.values(t[r]||{}).forEach((t={var r=e[0];if(t[0]===r){var n=t[1],i=e[3],o=e[2];n.apply(i,o)}}))}},7836:(e,t,r)={"use strict";r.d(t,{P:()=c,ee:()=u});var n=r(384),i=r(8990),o=r(3371),a=r(2646),s=r(5607);const c="nr@context:".concat(s.W),u=function e(t,r){var n={},s={},d={},l=!1;try{l=16===r.length&&(0,o.f)(r).isolatedBacklog}catch(e){}var f={on:p,addEventListener:p,removeEventListener:function(e,t){var r=n[e];if(!r)return;for(var i=0;i{s[n]=t,t in r||(r[t]=[])}))},abort:function(){f._aborted=!0,Object.keys(f.backlog).forEach((e={delete f.backlog[e]}))},isBuffering:function(e){return!!v()[s[e]]},debugId:r,backlog:l?{}:t&&"object"==typeof t.backlog?t.backlog:{},isolatedBacklog:l};return Object.defineProperty(f,"aborted",{get:()={let e=f._aborted||!1;return e||(t&&(e=t.aborted),e)}}),f;function h(e){return e&&e instanceof a.y?e:e?(0,i.I)(e,c,(()=new a.y(c))):new a.y(c)}function p(e,t){n[e]=g(e).concat(t)}function g(e){return n[e]||[]}function m(t){return d[t]=d[t]||e(f,t)}function v(){return f.backlog}}(void 0,"globalEE"),d=(0,n.Zm)();d.ee||(d.ee=u)},2646:(e,t,r)={"use strict";r.d(t,{y:()=n});class n{constructor(e){this.contextId=e}}},9908:(e,t,r)={"use strict";r.d(t,{d:()=n,p:()=i});var n=r(7836).ee.get("handle");function i(e,t,r,i,o){o?(o.buffer([e],i),o.emit(e,t,r)):(n.buffer([e],i),n.emit(e,t,r))}},3606:(e,t,r)={"use strict";r.d(t,{i:()=o});var n=r(9908);o.on=a;var i=o.handlers={};function o(e,t,r,o){a(o||n.d,i,e,t,r)}function a(e,t,r,i,o){o||(o="feature"),e||(e=n.d);var a=t[o]=t[o]||{};(a[r]=a[r]||[]).push([e,i])}},3878:(e,t,r)={"use strict";function n(e,t){return{capture:e,passive:!1,signal:t}}function i(e,t,r=!1,i){window.addEventListener(e,t,n(r,i))}function o(e,t,r=!1,i){document.addEventListener(e,t,n(r,i))}r.d(t,{DD:()=o,jT:()=n,sp:()=i})},5607:(e,t,r)={"use strict";r.d(t,{W:()=n});const n=(0,r(9566).bz)()},9566:(e,t,r)={"use strict";r.d(t,{LA:()=s,ZF:()=c,bz:()=a,el:()=u});var n=r(6154);const i="xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx";function o(e,t){return e?15&e[t]:16*Math.random()|0}function a(){const e=n.gm?.crypto||n.gm?.msCrypto;let t,r=0;return e&&e.getRandomValues&&(t=e.getRandomValues(new Uint8Array(30))),i.split("").map((e="x"===e?o(t,r++).toString(16):"y"===e?(3&o()|8).toString(16):e)).join("")}function s(e){const t=n.gm?.crypto||n.gm?.msCrypto;let r,i=0;t&&t.getRandomValues&&(r=t.getRandomValues(new Uint8Array(e)));const a=[];for(var s=0;s{"use strict";r.d(t,{BB:()=a,H3:()=n,g:()=u,iL:()=c,tS:()=s,uh:()=i,wk:()=o});const n="NRBA",i="SESSION",o=144e5,a=18e5,s={STARTED:"session-started",PAUSE:"session-pause",RESET:"session-reset",RESUME:"session-resume",UPDATE:"session-update"},c={SAME_TAB:"same-tab",CROSS_TAB:"cross-tab"},u={OFF:0,FULL:1,ERROR:2}},1863:(e,t,r)={"use strict";function n(){return Math.floor(performance.now())}r.d(t,{t:()=n})},7485:(e,t,r)={"use strict";r.d(t,{D:()=i});var n=r(6154);function i(e){if(0===(e||"").indexOf("data:"))return{protocol:"data"};try{const t=new URL(e,location.href),r={port:t.port,hostname:t.hostname,pathname:t.pathname,search:t.search,protocol:t.protocol.slice(0,t.protocol.indexOf(":")),sameOrigin:t.protocol===n.gm?.location?.protocol&&t.host===n.gm?.location?.host};return r.port&&""!==r.port||("http:"===t.protocol&&(r.port="80"),"https:"===t.protocol&&(r.port="443")),r.pathname&&""!==r.pathname?r.pathname.startsWith("/")||(r.pathname="/".concat(r.pathname)):r.pathname="/",r}catch(e){return{}}}},944:(e,t,r)={"use strict";function n(e,t){"function"==typeof console.debug&&console.debug("New Relic Warning: https://github.com/newrelic/newrelic-browser-agent/blob/main/docs/warning-codes.md#".concat(e),t)}r.d(t,{R:()=n})},5284:(e,t,r)={"use strict";r.d(t,{t:()=c,B:()=s});var n=r(7836),i=r(6154);const o="newrelic";const a=new Set,s={};function c(e,t){const r=n.ee.get(t);s[t]??={},e&&"object"==typeof e&&(a.has(t)||(r.emit("rumresp",[e]),s[t]=e,a.add(t),function(e={}){try{i.gm.dispatchEvent(new CustomEvent(o,{detail:e}))}catch(e){}}({loaded:!0})))}},8990:(e,t,r)={"use strict";r.d(t,{I:()=i});var n=Object.prototype.hasOwnProperty;function i(e,t,r){if(n.call(e,t))return e[t];var i=r();if(Object.defineProperty&&Object.keys)try{return Object.defineProperty(e,t,{value:i,writable:!0,enumerable:!1}),i}catch(e){}return e[t]=i,i}},6389:(e,t,r)={"use strict";function n(e,t=500,r={}){const n=r?.leading||!1;let i;return(...r)={n&&void 0===i&&(e.apply(this,r),i=setTimeout((()={i=clearTimeout(i)}),t)),n||(clearTimeout(i),i=setTimeout((()={e.apply(this,r)}),t))}}function i(e){let t=!1;return(...r)={t||(t=!0,e.apply(this,r))}}r.d(t,{J:()=i,s:()=n})},3304:(e,t,r)={"use strict";r.d(t,{A:()=o});var n=r(7836);const i=()={const e=new WeakSet;return(t,r)={if("object"==typeof r&&null!==r){if(e.has(r))return;e.add(r)}return r}};function o(e){try{return JSON.stringify(e,i())??""}catch(e){try{n.ee.emit("internal-error",[e])}catch(e){}return""}}},5289:(e,t,r)={"use strict";r.d(t,{GG:()=o,sB:()=a});var n=r(3878);function i(){return"undefined"==typeof document||"complete"===document.readyState}function o(e,t){if(i())return e();(0,n.sp)("load",e,t)}function a(e){if(i())return e();(0,n.DD)("DOMContentLoaded",e)}},384:(e,t,r)={"use strict";r.d(t,{NT:()=o,US:()=d,Zm:()=a,bQ:()=c,dV:()=s,nY:()=u,pV:()=l});var n=r(6154),i=r(1863);const o={beacon:"bam.nr-data.net",errorBeacon:"bam.nr-data.net"};function a(){return n.gm.NREUM||(n.gm.NREUM={}),void 0===n.gm.newrelic&&(n.gm.newrelic=n.gm.NREUM),n.gm.NREUM}function s(){let e=a();return e.o||(e.o={ST:n.gm.setTimeout,SI:n.gm.setImmediate,CT:n.gm.clearTimeout,XHR:n.gm.XMLHttpRequest,REQ:n.gm.Request,EV:n.gm.Event,PR:n.gm.Promise,MO:n.gm.MutationObserver,FETCH:n.gm.fetch,WS:n.gm.WebSocket}),e}function c(e,t){let r=a();r.initializedAgents??={},t.initializedAt={ms:(0,i.t)(),date:new Date},r.initializedAgents[e]=t}function u(e){let t=a();return t.initializedAgents?.[e]}function d(e,t){a()[e]=t}function l(){return function(){let e=a();const t=e.info||{};e.info={beacon:o.beacon,errorBeacon:o.errorBeacon,...t}}(),function(){let e=a();const t=e.init||{};e.init={...t}}(),s(),function(){let e=a();const t=e.loader_config||{};e.loader_config={...t}}(),a()}},2843:(e,t,r)={"use strict";r.d(t,{u:()=i});var n=r(3878);function i(e,t=!1,r,i){(0,n.DD)("visibilitychange",(function(){if(t)return void("hidden"===document.visibilityState&&e());e(document.visibilityState)}),r,i)}},8139:(e,t,r)={"use strict";r.d(t,{u:()=f});var n=r(7836),i=r(3434),o=r(8990),a=r(6154);const s={},c=a.gm.XMLHttpRequest,u="addEventListener",d="removeEventListener",l="nr@wrapped:".concat(n.P);function f(e){var t=function(e){return(e||n.ee).get("events")}(e);if(s[t.debugId]++)return t;s[t.debugId]=1;var r=(0,i.YM)(t,!0);function f(e){r.inPlace(e,[u,d],"-",p)}function p(e,t){return e[1]}return"getPrototypeOf"in Object&&(a.RI&&h(document,f),c&&h(c.prototype,f),h(a.gm,f)),t.on(u+"-start",(function(e,t){var n=e[1];if(null!==n&&("function"==typeof n||"object"==typeof n)){var i=(0,o.I)(n,l,(function(){var e={object:function(){if("function"!=typeof n.handleEvent)return;return n.handleEvent.apply(n,arguments)},function:n}[typeof n];return e?r(e,"fn-",null,e.name||"anonymous"):n}));this.wrapped=e[1]=i}})),t.on(d+"-start",(function(e){e[1]=this.wrapped||e[1]})),t}function h(e,t,...r){let n=e;for(;"object"==typeof n&&!Object.prototype.hasOwnProperty.call(n,u);)n=Object.getPrototypeOf(n);n&&t(n,...r)}},3434:(e,t,r)={"use strict";r.d(t,{Jt:()=o,YM:()=c});var n=r(7836),i=r(5607);const o="nr@original:".concat(i.W);var a=Object.prototype.hasOwnProperty,s=!1;function c(e,t){return e||(e=n.ee),r.inPlace=function(e,t,n,i,o){n||(n="");const a="-"===n.charAt(0);for(let s=0;s{"use strict";r.d(t,{J:()=c});var n=r(7836),i=r(2646),o=r(944),a=r(3434);const s=new Map;function c(e,t,r,c){if("object"!=typeof t||!t||"string"!=typeof r||!r||"function"!=typeof t[r])return(0,o.R)(29);const u=function(e){return(e||n.ee).get("logger")}(e),d=(0,a.YM)(u),l=new i.y(n.P);l.level=c.level,l.customAttributes=c.customAttributes;const f=t[r]?.[a.Jt]||t[r];return s.set(f,l),d.inPlace(t,[r],"wrap-logger-",(()=s.get(f))),u}},9300:(e,t,r)={"use strict";r.d(t,{T:()=n});const n=r(860).K7.ajax},3333:(e,t,r)={"use strict";r.d(t,{$v:()=u,TZ:()=n,Zp:()=i,kd:()=c,mq:()=s,nf:()=a,qN:()=o});const n=r(860).K7.genericEvents,i=["auxclick","click","copy","keydown","paste","scrollend"],o=["focus","blur"],a=4,s=1e3,c=["PageAction","UserAction","BrowserPerformance"],u={MARKS:"experimental.marks",MEASURES:"experimental.measures",RESOURCES:"experimental.resources"}},6774:(e,t,r)={"use strict";r.d(t,{T:()=n});const n=r(860).K7.jserrors},993:(e,t,r)={"use strict";r.d(t,{A$:()=o,ET:()=a,TZ:()=s,p_:()=i});var n=r(860);const i={ERROR:"ERROR",WARN:"WARN",INFO:"INFO",DEBUG:"DEBUG",TRACE:"TRACE"},o={OFF:0,ERROR:1,WARN:2,INFO:3,DEBUG:4,TRACE:5},a="log",s=n.K7.logging},3785:(e,t,r)={"use strict";r.d(t,{R:()=c,b:()=u});var n=r(9908),i=r(1863),o=r(860),a=r(8154),s=r(993);function c(e,t,r={},c=s.p_.INFO){(0,n.p)(a.xV,["API/logging/".concat(c.toLowerCase(),"/called")],void 0,o.K7.metrics,e),(0,n.p)(s.ET,[(0,i.t)(),t,r,c],void 0,o.K7.logging,e)}function u(e){return"string"==typeof e&&Object.values(s.p_).some((t=t===e.toUpperCase().trim()))}},8154:(e,t,r)={"use strict";r.d(t,{z_:()=o,XG:()=s,TZ:()=n,rs:()=i,xV:()=a});r(6154),r(9566),r(384);const n=r(860).K7.metrics,i="sm",o="cm",a="storeSupportabilityMetrics",s="storeEventMetrics"},6630:(e,t,r)={"use strict";r.d(t,{T:()=n});const n=r(860).K7.pageViewEvent},782:(e,t,r)={"use strict";r.d(t,{T:()=n});const n=r(860).K7.pageViewTiming},6344:(e,t,r)={"use strict";r.d(t,{BB:()=d,G4:()=o,Qb:()=l,TZ:()=i,Ug:()=a,_s:()=s,bc:()=u,yP:()=c});var n=r(2614);const i=r(860).K7.sessionReplay,o={RECORD:"recordReplay",PAUSE:"pauseReplay",REPLAY_RUNNING:"replayRunning",ERROR_DURING_REPLAY:"errorDuringReplay"},a=.12,s={DomContentLoaded:0,Load:1,FullSnapshot:2,IncrementalSnapshot:3,Meta:4,Custom:5},c={[n.g.ERROR]:15e3,[n.g.FULL]:3e5,[n.g.OFF]:0},u={RESET:{message:"Session was reset",sm:"Reset"},IMPORT:{message:"Recorder failed to import",sm:"Import"},TOO_MANY:{message:"429: Too Many Requests",sm:"Too-Many"},TOO_BIG:{message:"Payload was too large",sm:"Too-Big"},CROSS_TAB:{message:"Session Entity was set to OFF on another tab",sm:"Cross-Tab"},ENTITLEMENTS:{message:"Session Replay is not allowed and will not be started",sm:"Entitlement"}},d=5e3,l={API:"api"}},5270:(e,t,r)={"use strict";r.d(t,{Aw:()=c,CT:()=u,SR:()=s});var n=r(384),i=r(9417),o=r(7767),a=r(6154);function s(e){return!!(0,n.dV)().o.MO&&(0,o.V)(e)&&!0===(0,i.gD)(e,"session_trace.enabled")}function c(e){return!0===(0,i.gD)(e,"session_replay.preload")&&s(e)}function u(e,t){const r=t.correctAbsoluteTimestamp(e);return{originalTimestamp:e,correctedTimestamp:r,timestampDiff:e-r,originTime:a.WN,correctedOriginTime:t.correctedOriginTime,originTimeDiff:Math.floor(a.WN-t.correctedOriginTime)}}},3738:(e,t,r)={"use strict";r.d(t,{He:()=i,Kp:()=s,Lc:()=u,Rz:()=d,TZ:()=n,bD:()=o,d3:()=a,jx:()=l,uP:()=c});const n=r(860).K7.sessionTrace,i="bstResource",o="resource",a="-start",s="-end",c="fn"+a,u="fn"+s,d="pushState",l=1e3},3962:(e,t,r)={"use strict";r.d(t,{AM:()=o,O2:()=c,Qu:()=u,TZ:()=s,ih:()=d,pP:()=a,tC:()=i});var n=r(860);const i=["click","keydown","submit","popstate"],o="api",a="initialPageLoad",s=n.K7.softNav,c={INITIAL_PAGE_LOAD:"",ROUTE_CHANGE:1,UNSPECIFIED:2},u={INTERACTION:1,AJAX:2,CUSTOM_END:3,CUSTOM_TRACER:4},d={IP:"in progress",FIN:"finished",CAN:"cancelled"}},7378:(e,t,r)={"use strict";r.d(t,{$p:()=x,BR:()=b,Kp:()=R,L3:()=y,Lc:()=c,NC:()=o,SG:()=d,TZ:()=i,U6:()=p,UT:()=m,d3:()=w,dT:()=f,e5:()=A,gx:()=v,l9:()=l,oW:()=h,op:()=g,rw:()=u,tH:()=T,uP:()=s,wW:()=E,xq:()=a});var n=r(384);const i=r(860).K7.spa,o=["click","submit","keypress","keydown","keyup","change"],a=999,s="fn-start",c="fn-end",u="cb-start",d="api-ixn-",l="remaining",f="interaction",h="spaNode",p="jsonpNode",g="fetch-start",m="fetch-done",v="fetch-body-",b="jsonp-end",y=(0,n.dV)().o.ST,w="-start",R="-end",x="-body",E="cb"+R,A="jsTime",T="fetch"},4234:(e,t,r)={"use strict";r.d(t,{W:()=o});var n=r(7836),i=r(1687);class o{constructor(e,t){this.agentIdentifier=e,this.ee=n.ee.get(e),this.featureName=t,this.blocked=!1}deregisterDrain(){(0,i.x3)(this.agentIdentifier,this.featureName)}}},7767:(e,t,r)={"use strict";r.d(t,{V:()=o});var n=r(9417),i=r(6154);const o=e=i.RI&&!0===(0,n.gD)(e,"privacy.cookies_enabled")},8969:(e,t,r)={"use strict";r.d(t,{j:()=O});var n=r(860),i=r(2555),o=r(3371),a=r(9908),s=r(7836),c=r(1687),u=r(5289),d=r(6154),l=r(944),f=r(8154),h=r(384),p=r(6344);const g=["setErrorHandler","finished","addToTrace","addRelease","recordCustomEvent","addPageAction","setCurrentRouteName","setPageViewName","setCustomAttribute","interaction","noticeError","setUserId","setApplicationVersion","start",p.G4.RECORD,p.G4.PAUSE,"log","wrapLogger"],m=["setErrorHandler","finished","addToTrace","addRelease"];var v=r(1863),b=r(2614),y=r(993),w=r(3785),R=r(9414);function x(){const e=(0,h.pV)();g.forEach((t={e[t]=(...r)=function(t,...r){let n=[];return Object.values(e.initializedAgents).forEach((e={e&&e.api?e.exposed&&e.api[t]&&n.push(e.api[t](...r)):(0,l.R)(38,t)})),n.length1?n:n[0]}(t,...r)}))}const E={};var A=r(9417),T=r(5603),N=r(5284);const S=e={const t=e.startsWith("http");e+="/",r.p=t?e:"https://"+e};let _=!1;function O(e,t={},g,O){let{init:I,info:P,loader_config:j,runtime:C={},exposed:k=!0}=t;C.loaderType=g;const L=(0,h.pV)();P||(I=L.init,P=L.info,j=L.loader_config),(0,A.xN)(e.agentIdentifier,I||{}),(0,T.a)(e.agentIdentifier,j||{}),P.jsAttributes??={},d.bv&&(P.jsAttributes.isWorker=!0),(0,i.x1)(e.agentIdentifier,P);const H=(0,A.D0)(e.agentIdentifier),M=[P.beacon,P.errorBeacon];_||(H.proxy.assets&&(S(H.proxy.assets),M.push(H.proxy.assets)),H.proxy.beacon&&M.push(H.proxy.beacon),x(),(0,h.US)("activatedFeatures",N.B),e.runSoftNavOverSpa&&=!0===H.soft_navigations.enabled&&H.feature_flags.includes("soft_nav")),C.denyList=[...H.ajax.deny_list||[],...H.ajax.block_internal?M:[]],C.ptid=e.agentIdentifier,(0,o.V)(e.agentIdentifier,C),e.ee=s.ee.get(e.agentIdentifier),void 0===e.api&&(e.api=function(e,t,h=!1){t||(0,c.Ak)(e,"api");const g={};var x=s.ee.get(e),A=x.get("tracer");E[e]=b.g.OFF,x.on(p.G4.REPLAY_RUNNING,(t={E[e]=t}));var T="api-",N=T+"ixn-";function S(t,r,n,o){const a=(0,i.Vp)(e);return null===r?delete a.jsAttributes[t]:(0,i.x1)(e,{...a,jsAttributes:{...a.jsAttributes,[t]:r}}),I(T,n,!0,o||null===r?"session":void 0)(t,r)}function _(){}g.log=function(e,{customAttributes:t={},level:r=y.p_.INFO}={}){(0,a.p)(f.xV,["API/log/called"],void 0,n.K7.metrics,x),(0,w.R)(x,e,t,r)},g.wrapLogger=(e,t,{customAttributes:r={},level:i=y.p_.INFO}={})={(0,a.p)(f.xV,["API/wrapLogger/called"],void 0,n.K7.metrics,x),(0,R.J)(x,e,t,{customAttributes:r,level:i})},m.forEach((e={g[e]=I(T,e,!0,"api")})),g.addPageAction=I(T,"addPageAction",!0,n.K7.genericEvents),g.recordCustomEvent=I(T,"recordCustomEvent",!0,n.K7.genericEvents),g.setPageViewName=function(t,r){if("string"==typeof t)return"/"!==t.charAt(0)&&(t="/"+t),(0,o.f)(e).customTransaction=(r||"http://custom.transaction")+t,I(T,"setPageViewName",!0)()},g.setCustomAttribute=function(e,t,r=!1){if("string"==typeof e){if(["string","number","boolean"].includes(typeof t)||null===t)return S(e,t,"setCustomAttribute",r);(0,l.R)(40,typeof t)}else(0,l.R)(39,typeof e)},g.setUserId=function(e){if("string"==typeof e||null===e)return S("enduser.id",e,"setUserId",!0);(0,l.R)(41,typeof e)},g.setApplicationVersion=function(e){if("string"==typeof e||null===e)return S("application.version",e,"setApplicationVersion",!1);(0,l.R)(42,typeof e)},g.start=()={try{(0,a.p)(f.xV,["API/start/called"],void 0,n.K7.metrics,x),x.emit("manual-start-all")}catch(e){(0,l.R)(23,e)}},g[p.G4.RECORD]=function(){(0,a.p)(f.xV,["API/recordReplay/called"],void 0,n.K7.metrics,x),(0,a.p)(p.G4.RECORD,[],void 0,n.K7.sessionReplay,x)},g[p.G4.PAUSE]=function(){(0,a.p)(f.xV,["API/pauseReplay/called"],void 0,n.K7.metrics,x),(0,a.p)(p.G4.PAUSE,[],void 0,n.K7.sessionReplay,x)},g.interaction=function(e){return(new _).get("object"==typeof e?e:{})};const O=_.prototype={createTracer:function(e,t){var r={},i=this,o="function"==typeof t;return(0,a.p)(f.xV,["API/createTracer/called"],void 0,n.K7.metrics,x),h||(0,a.p)(N+"tracer",[(0,v.t)(),e,r],i,n.K7.spa,x),function(){if(A.emit((o?"":"no-")+"fn-start",[(0,v.t)(),i,o],r),o)try{return t.apply(this,arguments)}catch(e){const t="string"==typeof e?new Error(e):e;throw A.emit("fn-err",[arguments,this,t],r),t}finally{A.emit("fn-end",[(0,v.t)()],r)}}}};function I(e,t,r,i){return function(){return(0,a.p)(f.xV,["API/"+t+"/called"],void 0,n.K7.metrics,x),i&&(0,a.p)(e+t,[r?(0,v.t)():performance.now(),...arguments],r?null:this,i,x),r?void 0:this}}function P(){r.e(478).then(r.bind(r,8778)).then((({setAPI:t})={t(e),(0,c.Ze)(e,"api")})).catch((e={(0,l.R)(27,e),x.abort()}))}return["actionText","setName","setAttribute","save","ignore","onEnd","getContext","end","get"].forEach((e={O[e]=I(N,e,void 0,h?n.K7.softNav:n.K7.spa)})),g.setCurrentRouteName=h?I(N,"routeName",void 0,n.K7.softNav):I(T,"routeName",!0,n.K7.spa),g.noticeError=function(t,r){"string"==typeof t&&(t=new Error(t)),(0,a.p)(f.xV,["API/noticeError/called"],void 0,n.K7.metrics,x),(0,a.p)("err",[t,(0,v.t)(),!1,r,!!E[e]],void 0,n.K7.jserrors,x)},d.RI?(0,u.GG)((()=P()),!0):P(),g}(e.agentIdentifier,O,e.runSoftNavOverSpa)),void 0===e.exposed&&(e.exposed=k),_=!0}},8374:(e,t,r)={r.nc=(()={try{return document?.currentScript?.nonce}catch(e){}return""})()},860:(e,t,r)={"use strict";r.d(t,{$J:()=u,K7:()=s,P3:()=c,XX:()=i,qY:()=n,v4:()=a});const n="events",i="jserrors",o="browser/blobs",a="rum",s={ajax:"ajax",genericEvents:"generic_events",jserrors:i,logging:"logging",metrics:"metrics",pageAction:"page_action",pageViewEvent:"page_view_event",pageViewTiming:"page_view_timing",sessionReplay:"session_replay",sessionTrace:"session_trace",softNav:"soft_navigations",spa:"spa"},c={[s.pageViewEvent]:1,[s.pageViewTiming]:2,[s.metrics]:3,[s.jserrors]:4,[s.spa]:5,[s.ajax]:6,[s.sessionTrace]:7,[s.softNav]:8,[s.sessionReplay]:9,[s.logging]:10,[s.genericEvents]:11},u={[s.pageViewEvent]:a,[s.pageViewTiming]:n,[s.ajax]:n,[s.spa]:n,[s.softNav]:n,[s.metrics]:i,[s.jserrors]:i,[s.sessionTrace]:o,[s.sessionReplay]:o,[s.logging]:"browser/logs",[s.genericEvents]:"ins"}}},n={};function i(e){var t=n[e];if(void 0!==t)return t.exports;var o=n[e]={exports:{}};return r[e](o,o.exports,i),o.exports}i.m=r,i.d=(e,t)={for(var r in t)i.o(t,r)&&!i.o(e,r)&&Object.defineProperty(e,r,{enumerable:!0,get:t[r]})},i.f={},i.e=e=Promise.all(Object.keys(i.f).reduce(((t,r)=(i.f[r](e,t),t)),[])),i.u=e=({212:"nr-spa-compressor",249:"nr-spa-recorder",478:"nr-spa"}[e]+"-1.283.2.min.js"),i.o=(e,t)=Object.prototype.hasOwnProperty.call(e,t),e={},t="NRBA-1.283.2.PROD:",i.l=(r,n,o,a)={if(e[r])e[r].push(n);else{var s,c;if(void 0!==o)for(var u=document.getElementsByTagName("script"),d=0;d{s.onerror=s.onload=null,clearTimeout(p);var i=e[r];if(delete e[r],s.parentNode&&s.parentNode.removeChild(s),i&&i.forEach((e=e(n))),t)return t(n)},p=setTimeout(h.bind(null,void 0,{type:"timeout",target:s}),12e4);s.onerror=h.bind(null,s.onerror),s.onload=h.bind(null,s.onload),c&&document.head.appendChild(s)}},i.r=e={"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},i.p="https://js-agent.newrelic.com/",(()={var e={38:0,788:0};i.f.j=(t,r)={var n=i.o(e,t)?e[t]:void 0;if(0!==n)if(n)r.push(n[2]);else{var o=new Promise(((r,i)=n=e[t]=[r,i]));r.push(n[2]=o);var a=i.p+i.u(t),s=new Error;i.l(a,(r={if(i.o(e,t)&&(0!==(n=e[t])&&(e[t]=void 0),n)){var o=r&&("load"===r.type?"missing":r.type),a=r&&r.target&&r.target.src;s.message="Loading chunk "+t+" failed.\n("+o+": "+a+")",s.name="ChunkLoadError",s.type=o,s.request=a,n[1](s)}}),"chunk-"+t,t)}};var t=(t,r)={var n,o,[a,s,c]=r,u=0;if(a.some((t=0!==e[t]))){for(n in s)i.o(s,n)&&(i.m[n]=s[n]);if(c)c(i)}for(t&&t(r);u{"use strict";i(8374);var e=i(944),t=i(6344),r=i(9566);class n{agentIdentifier;constructor(){this.agentIdentifier=(0,r.LA)(16)}#e(t,...r){if("function"==typeof this.api?.[t])return this.api[t](...r);(0,e.R)(35,t)}addPageAction(e,t){return this.#e("addPageAction",e,t)}recordCustomEvent(e,t){return this.#e("recordCustomEvent",e,t)}setPageViewName(e,t){return this.#e("setPageViewName",e,t)}setCustomAttribute(e,t,r){return this.#e("setCustomAttribute",e,t,r)}noticeError(e,t){return this.#e("noticeError",e,t)}setUserId(e){return this.#e("setUserId",e)}setApplicationVersion(e){return this.#e("setApplicationVersion",e)}setErrorHandler(e){return this.#e("setErrorHandler",e)}addRelease(e,t){return this.#e("addRelease",e,t)}log(e,t){return this.#e("log",e,t)}}class o extends n{#e(t,...r){if("function"==typeof this.api?.[t])return this.api[t](...r);(0,e.R)(35,t)}start(){return this.#e("start")}finished(e){return this.#e("finished",e)}recordReplay(){return this.#e(t.G4.RECORD)}pauseReplay(){return this.#e(t.G4.PAUSE)}addToTrace(e){return this.#e("addToTrace",e)}setCurrentRouteName(e){return this.#e("setCurrentRouteName",e)}interaction(){return this.#e("interaction")}wrapLogger(e,t,r){return this.#e("wrapLogger",e,t,r)}}var a=i(860),s=i(9417);const c=Object.values(a.K7);function u(e){const t={};return c.forEach((r={t[r]=function(e,t){return!0===(0,s.gD)(t,"".concat(e,".enabled"))}(r,e)})),t}var d=i(8969);var l=i(1687),f=i(4234),h=i(5289),p=i(6154),g=i(5270),m=i(7767),v=i(6389);class b extends f.W{constructor(e,t,r=!0){super(e.agentIdentifier,t),this.auto=r,this.abortHandler=void 0,this.featAggregate=void 0,this.onAggregateImported=void 0,!1===e.init[this.featureName].autoStart&&(this.auto=!1),this.auto?(0,l.Ak)(e.agentIdentifier,t):this.ee.on("manual-start-all",(0,v.J)((()={(0,l.Ak)(e.agentIdentifier,this.featureName),this.auto=!0,this.importAggregator(e)})))}importAggregator(t,r={}){if(this.featAggregate||!this.auto)return;let n;this.onAggregateImported=new Promise((e={n=e}));const o=async()={let o;try{if((0,m.V)(this.agentIdentifier)){const{setupAgentSession:e}=await i.e(478).then(i.bind(i,6526));o=e(t)}}catch(t){(0,e.R)(20,t),this.ee.emit("internal-error",[t]),this.featureName===a.K7.sessionReplay&&this.abortHandler?.()}try{if(!this.#t(this.featureName,o))return(0,l.Ze)(this.agentIdentifier,this.featureName),void n(!1);const{lazyFeatureLoader:e}=await i.e(478).then(i.bind(i,6103)),{Aggregate:a}=await e(this.featureName,"aggregate");this.featAggregate=new a(t,r),t.runtime.harvester.initializedAggregates.push(this.featAggregate),n(!0)}catch(t){(0,e.R)(34,t),this.abortHandler?.(),(0,l.Ze)(this.agentIdentifier,this.featureName,!0),n(!1),this.ee&&this.ee.abort()}};p.RI?(0,h.GG)((()=o()),!0):o()}#t(e,t){switch(e){case a.K7.sessionReplay:return(0,g.SR)(this.agentIdentifier)&&!!t;case a.K7.sessionTrace:return!!t;default:return!0}}}var y=i(6630);class w extends b{static featureName=y.T;constructor(e,t=!0){super(e,y.T,t),this.importAggregator(e)}}var R=i(384);var x=i(9908),E=i(2843),A=i(3878),T=i(782),N=i(1863);class S extends b{static featureName=T.T;constructor(e,t=!0){super(e,T.T,t),p.RI&&((0,E.u)((()=(0,x.p)("docHidden",[(0,N.t)()],void 0,T.T,this.ee)),!0),(0,A.sp)("pagehide",(()=(0,x.p)("winPagehide",[(0,N.t)()],void 0,T.T,this.ee))),this.importAggregator(e))}}var _=i(8154);class O extends b{static featureName=_.TZ;constructor(e,t=!0){super(e,_.TZ,t),this.importAggregator(e)}}var I=i(6774),P=i(3304);class j{constructor(e,t,r,n,i){this.name="UncaughtError",this.message="string"==typeof e?e:(0,P.A)(e),this.sourceURL=t,this.line=r,this.column=n,this.__newrelic=i}}function C(e){return H(e)?e:new j(void 0!==e?.message?e.message:e,e?.filename||e?.sourceURL,e?.lineno||e?.line,e?.colno||e?.col,e?.__newrelic)}function k(e){const t="Unhandled Promise Rejection";if(!e?.reason)return;if(H(e.reason))try{return e.reason.message=t+": "+e.reason.message,C(e.reason)}catch(t){return C(e.reason)}const r=C(e.reason);return r.message=t+": "+r?.message,r}function L(e){if(e.error instanceof SyntaxError&&!/:\d+$/.test(e.error.stack?.trim())){const t=new j(e.message,e.filename,e.lineno,e.colno,e.error.__newrelic);return t.name=SyntaxError.name,t}return H(e.error)?e.error:C(e)}function H(e){return e instanceof Error&&!!e.stack}class M extends b{static featureName=I.T;#r=!1;constructor(e,r=!0){super(e,I.T,r);try{this.removeOnAbort=new AbortController}catch(e){}this.ee.on("internal-error",((e,t)={this.abortHandler&&(0,x.p)("ierr",[C(e),(0,N.t)(),!0,{},this.#r,t],void 0,this.featureName,this.ee)})),this.ee.on(t.G4.REPLAY_RUNNING,(e={this.#r=e})),p.gm.addEventListener("unhandledrejection",(e={this.abortHandler&&(0,x.p)("err",[k(e),(0,N.t)(),!1,{unhandledPromiseRejection:1},this.#r],void 0,this.featureName,this.ee)}),(0,A.jT)(!1,this.removeOnAbort?.signal)),p.gm.addEventListener("error",(e={this.abortHandler&&(0,x.p)("err",[L(e),(0,N.t)(),!1,{},this.#r],void 0,this.featureName,this.ee)}),(0,A.jT)(!1,this.removeOnAbort?.signal)),this.abortHandler=this.#n,this.importAggregator(e)}#n(){this.removeOnAbort?.abort(),this.abortHandler=void 0}}var D=i(8990);let K=1;const U="nr@id";function V(e){const t=typeof e;return!e||"object"!==t&&"function"!==t?-1:e===p.gm?0:(0,D.I)(e,U,(function(){return K++}))}function G(e){if("string"==typeof e&&e.length)return e.length;if("object"==typeof e){if("undefined"!=typeof ArrayBuffer&&e instanceof ArrayBuffer&&e.byteLength)return e.byteLength;if("undefined"!=typeof Blob&&e instanceof Blob&&e.size)return e.size;if(!("undefined"!=typeof FormData&&e instanceof FormData))try{return(0,P.A)(e).length}catch(e){return}}}var F=i(8139),B=i(7836),W=i(3434);const z={},q=["open","send"];function Z(t){var r=t||B.ee;const n=function(e){return(e||B.ee).get("xhr")}(r);if(void 0===p.gm.XMLHttpRequest)return n;if(z[n.debugId]++)return n;z[n.debugId]=1,(0,F.u)(r);var i=(0,W.YM)(n),o=p.gm.XMLHttpRequest,a=p.gm.MutationObserver,s=p.gm.Promise,c=p.gm.setInterval,u="readystatechange",d=["onload","onerror","onabort","onloadstart","onloadend","onprogress","ontimeout"],l=[],f=p.gm.XMLHttpRequest=function(t){const r=new o(t),a=n.context(r);try{n.emit("new-xhr",[r],a),r.addEventListener(u,(s=a,function(){var e=this;e.readyState3&&!s.resolved&&(s.resolved=!0,n.emit("xhr-resolved",[],e)),i.inPlace(e,d,"fn-",y)}),(0,A.jT)(!1))}catch(t){(0,e.R)(15,t);try{n.emit("internal-error",[t])}catch(e){}}var s;return r};function h(e,t){i.inPlace(t,["onreadystatechange"],"fn-",y)}if(function(e,t){for(var r in e)t[r]=e[r]}(o,f),f.prototype=o.prototype,i.inPlace(f.prototype,q,"-xhr-",y),n.on("send-xhr-start",(function(e,t){h(e,t),function(e){l.push(e),a&&(g?g.then(b):c?c(b):(m=-m,v.data=m))}(t)})),n.on("open-xhr-start",h),a){var g=s&&s.resolve();if(!c&&!s){var m=1,v=document.createTextNode(m);new a(b).observe(v,{characterData:!0})}}else r.on("fn-end",(function(e){e[0]&&e[0].type===u||b()}));function b(){for(var e=0;e{r(Q[te],e,J),r(ee[te],e,J)})),r(p.gm,"fetch",Y),t.on(Y+"end",(function(e,r){var n=this;if(r){var i=r.headers.get("content-length");null!==i&&(n.rxSize=i),t.emit(Y+"done",[null,r],n)}else t.emit(Y+"done",[e],n)})),t}var ie=i(7485),oe=i(5603);class ae{constructor(e){this.agentIdentifier=e}generateTracePayload(e){if(!this.shouldGenerateTrace(e))return null;var t=(0,oe.o)(this.agentIdentifier);if(!t)return null;var n=(t.accountID||"").toString()||null,i=(t.agentID||"").toString()||null,o=(t.trustKey||"").toString()||null;if(!n||!i)return null;var a=(0,r.ZF)(),s=(0,r.el)(),c=Date.now(),u={spanId:a,traceId:s,timestamp:c};return(e.sameOrigin||this.isAllowedOrigin(e)&&this.useTraceContextHeadersForCors())&&(u.traceContextParentHeader=this.generateTraceContextParentHeader(a,s),u.traceContextStateHeader=this.generateTraceContextStateHeader(a,c,n,i,o)),(e.sameOrigin&&!this.excludeNewrelicHeader()||!e.sameOrigin&&this.isAllowedOrigin(e)&&this.useNewrelicHeaderForCors())&&(u.newrelicHeader=this.generateTraceHeader(a,s,c,n,i,o)),u}generateTraceContextParentHeader(e,t){return"00-"+t+"-"+e+"-01"}generateTraceContextStateHeader(e,t,r,n,i){return i+"@nr=0-1-"+r+"-"+n+"-"+e+"----"+t}generateTraceHeader(e,t,r,n,i,o){if(!("function"==typeof p.gm?.btoa))return null;var a={v:[0,1],d:{ty:"Browser",ac:n,ap:i,id:e,tr:t,ti:r}};return o&&n!==o&&(a.d.tk=o),btoa((0,P.A)(a))}shouldGenerateTrace(e){return this.isDtEnabled()&&this.isAllowedOrigin(e)}isAllowedOrigin(e){var t=!1,r={};if((0,s.gD)(this.agentIdentifier,"distributed_tracing")&&(r=(0,s.D0)(this.agentIdentifier).distributed_tracing),e.sameOrigin)t=!0;else if(r.allowed_origins instanceof Array)for(var n=0;n(0,x.p)(e,t,r,n,this.ee);try{const e={xmlhttprequest:"xhr",fetch:"fetch",beacon:"beacon"};p.gm?.performance?.getEntriesByType("resource").forEach((t={if(t.initiatorType in e&&0!==t.responseStatus){const r={status:t.responseStatus},n={rxSize:t.transferSize,duration:Math.floor(t.duration),cbTime:0};pe(r,t.name),this.handler("xhr",[r,n,t.startTime,t.responseEnd,e[t.initiatorType]],void 0,a.K7.ajax)}}))}catch(e){}ne(this.ee),Z(this.ee),function(e,t,r,n){function i(e){var t=this;t.totalCbs=0,t.called=0,t.cbTime=0,t.end=R,t.ended=!1,t.xhrGuids={},t.lastSize=null,t.loadCaptureCalled=!1,t.params=this.params||{},t.metrics=this.metrics||{},e.addEventListener("load",(function(r){E(t,e)}),(0,A.jT)(!1)),p.lR||e.addEventListener("progress",(function(e){t.lastSize=e.loaded}),(0,A.jT)(!1))}function o(e){this.params={method:e[0]},pe(this,e[1]),this.metrics={}}function s(t,r){e.loader_config.xpid&&this.sameOrigin&&r.setRequestHeader("X-NewRelic-ID",e.loader_config.xpid);var i=n.generateTracePayload(this.parsedOrigin);if(i){var o=!1;i.newrelicHeader&&(r.setRequestHeader("newrelic",i.newrelicHeader),o=!0),i.traceContextParentHeader&&(r.setRequestHeader("traceparent",i.traceContextParentHeader),i.traceContextStateHeader&&r.setRequestHeader("tracestate",i.traceContextStateHeader),o=!0),o&&(this.dt=i)}}function c(e,r){var n=this.metrics,i=e[0],o=this;if(n&&i){var a=G(i);a&&(n.txSize=a)}this.startTime=(0,N.t)(),this.body=i,this.listener=function(e){try{"abort"!==e.type||o.loadCaptureCalled||(o.params.aborted=!0),("load"!==e.type||o.called===o.totalCbs&&(o.onloadCalled||"function"!=typeof r.onload)&&"function"==typeof o.end)&&o.end(r)}catch(e){try{t.emit("internal-error",[e])}catch(e){}}};for(var s=0;s1?e[1]=o:e.push(o)}}function s(e,t){var r=!1;return t.newrelicHeader&&(e.set("newrelic",t.newrelicHeader),r=!0),t.traceContextParentHeader&&(e.set("traceparent",t.traceContextParentHeader),t.traceContextStateHeader&&e.set("tracestate",t.traceContextStateHeader),r=!0),r}}function y(e,t){this.params={},this.metrics={},this.startTime=(0,N.t)(),this.dt=t,e.length=1&&(this.target=e[0]),e.length=2&&(this.opts=e[1]);var r,n=this.opts||{},i=this.target;"string"==typeof i?r=i:"object"==typeof i&&i instanceof le?r=i.url:p.gm?.URL&&"object"==typeof i&&i instanceof URL&&(r=i.href),pe(this,r);var o=(""+(i&&i instanceof le&&i.method||n.method||"GET")).toUpperCase();this.params.method=o,this.body=n.body,this.txSize=G(n.body)||0}function w(e,t){if(this.endTime=(0,N.t)(),this.params||(this.params={}),(0,ce.iW)(this.params))return;let n;this.params.status=t?t.status:0,"string"==typeof this.rxSize&&this.rxSize.length0&&(n=+this.rxSize);const i={txSize:this.txSize,rxSize:n,duration:(0,N.t)()-this.startTime};r("xhr",[this.params,i,this.startTime,this.endTime,"fetch"],this,a.K7.ajax)}function R(e){const t=this.params,n=this.metrics;if(!this.ended){this.ended=!0;for(let t=0;t{const t=e.getEntries();(0,x.p)(ye,[t],void 0,a.K7.sessionTrace,r)})),n.observe({type:we,buffered:!0})}catch(e){}this.importAggregator(e,{resourceObserver:n})}}var _e=i(2614);class Oe extends b{static featureName=t.TZ;#i;#o;constructor(e,r=!0){let n;super(e,t.TZ,r),this.replayRunning=!1,this.#o=e;try{n=JSON.parse(localStorage.getItem("".concat(_e.H3,"_").concat(_e.uh)))}catch(e){}(0,g.SR)(e.agentIdentifier)&&this.ee.on(t.G4.RECORD,(()=this.#a())),this.#s(n)?(this.#i=n?.sessionReplayMode,this.#c()):this.importAggregator(e),this.ee.on("err",(e={this.replayRunning&&(this.errorNoticed=!0,(0,x.p)(t.G4.ERROR_DURING_REPLAY,[e],void 0,this.featureName,this.ee))})),this.ee.on(t.G4.REPLAY_RUNNING,(e={this.replayRunning=e}))}#s(e){return e&&(e.sessionReplayMode===_e.g.FULL||e.sessionReplayMode===_e.g.ERROR)||(0,g.Aw)(this.agentIdentifier)}#u=!1;async#c(e){if(!this.#u){this.#u=!0;try{const{Recorder:t}=await Promise.all([i.e(478),i.e(249)]).then(i.bind(i,8589));this.recorder??=new t({mode:this.#i,agentIdentifier:this.agentIdentifier,trigger:e,ee:this.ee,agentRef:this.#o}),this.recorder.startRecording(),this.abortHandler=this.recorder.stopRecording}catch(e){}this.importAggregator(this.#o,{recorder:this.recorder,errorNoticed:this.errorNoticed})}}#a(){this.featAggregate?this.featAggregate.mode!==_e.g.FULL&&this.featAggregate.initializeRecording(_e.g.FULL,!0):(this.#i=_e.g.FULL,this.#c(t.Qb.API),this.recorder&&this.recorder.parent.mode!==_e.g.FULL&&(this.recorder.parent.mode=_e.g.FULL,this.recorder.stopRecording(),this.recorder.startRecording(),this.abortHandler=this.recorder.stopRecording))}}var Ie=i(3962);class Pe extends b{static featureName=Ie.TZ;constructor(e,t=!0){if(super(e,Ie.TZ,t),!p.RI||!(0,R.dV)().o.MO)return;const r=ve(this.ee);Ie.tC.forEach((e={(0,A.sp)(e,(e={a(e)}),!0)}));const n=()=(0,x.p)("newURL",[(0,N.t)(),""+window.location],void 0,this.featureName,this.ee);r.on("pushState-end",n),r.on("replaceState-end",n);try{this.removeOnAbort=new AbortController}catch(e){}(0,A.sp)("popstate",(e=(0,x.p)("newURL",[e.timeStamp,""+window.location],void 0,this.featureName,this.ee)),!0,this.removeOnAbort?.signal);let i=!1;const o=new((0,R.dV)().o.MO)(((e,t)={i||(i=!0,requestAnimationFrame((()={(0,x.p)("newDom",[(0,N.t)()],void 0,this.featureName,this.ee),i=!1})))})),a=(0,v.s)((e={(0,x.p)("newUIEvent",[e],void 0,this.featureName,this.ee),o.observe(document.body,{attributes:!0,childList:!0,subtree:!0,characterData:!0})}),100,{leading:!0});this.abortHandler=function(){this.removeOnAbort?.abort(),o.disconnect(),this.abortHandler=void 0},this.importAggregator(e,{domObserver:o})}}var je=i(7378);const Ce={},ke=["appendChild","insertBefore","replaceChild"];function Le(e){const t=function(e){return(e||B.ee).get("jsonp")}(e);if(!p.RI||Ce[t.debugId])return t;Ce[t.debugId]=!0;var r=(0,W.YM)(t),n=/[?&](?:callback|cb)=([^&#]+)/,i=/(.*)\.([^.]+)/,o=/^(\w+)(\.|$)(.*)$/;function a(e,t){if(!e)return t;const r=e.match(o),n=r[1];return a(r[3],t[n])}return r.inPlace(Node.prototype,ke,"dom-"),t.on("dom-start",(function(e){!function(e){if(!e||"string"!=typeof e.nodeName||"script"!==e.nodeName.toLowerCase())return;if("function"!=typeof e.addEventListener)return;var o=(s=e.src,c=s.match(n),c?c[1]:null);var s,c;if(!o)return;var u=function(e){var t=e.match(i);if(t&&t.length=3)return{key:t[2],parent:a(t[1],window)};return{key:e,parent:window}}(o);if("function"!=typeof u.parent[u.key])return;var d={};function l(){t.emit("jsonp-end",[],d),e.removeEventListener("load",l,(0,A.jT)(!1)),e.removeEventListener("error",f,(0,A.jT)(!1))}function f(){t.emit("jsonp-error",[],d),t.emit("jsonp-end",[],d),e.removeEventListener("load",l,(0,A.jT)(!1)),e.removeEventListener("error",f,(0,A.jT)(!1))}r.inPlace(u.parent,[u.key],"cb-",d),e.addEventListener("load",l,(0,A.jT)(!1)),e.addEventListener("error",f,(0,A.jT)(!1)),t.emit("new-jsonp",[e.src],d)}(e[0])})),t}const He={};function Me(e){const t=function(e){return(e||B.ee).get("promise")}(e);if(He[t.debugId])return t;He[t.debugId]=!0;var r=t.context,n=(0,W.YM)(t),i=p.gm.Promise;return i&&function(){function e(r){var o=t.context(),a=n(r,"executor-",o,null,!1);const s=Reflect.construct(i,[a],e);return t.context(s).getCtx=function(){return o},s}p.gm.Promise=e,Object.defineProperty(e,"name",{value:"Promise"}),e.toString=function(){return i.toString()},Object.setPrototypeOf(e,i),["all","race"].forEach((function(r){const n=i[r];e[r]=function(e){let i=!1;[...e||[]].forEach((e={this.resolve(e).then(a("all"===r),a(!1))}));const o=n.apply(this,arguments);return o;function a(e){return function(){t.emit("propagate",[null,!i],o,!1,!1),i=i||!e}}}})),["resolve","reject"].forEach((function(r){const n=i[r];e[r]=function(e){const r=n.apply(this,arguments);return e!==r&&t.emit("propagate",[e,!0],r,!1,!1),r}})),e.prototype=i.prototype;const o=i.prototype.then;i.prototype.then=function(...e){var i=this,a=r(i);a.promise=i,e[0]=n(e[0],"cb-",a,null,!1),e[1]=n(e[1],"cb-",a,null,!1);const s=o.apply(this,e);return a.nextPromise=s,t.emit("propagate",[i,!0],s,!1,!1),s},i.prototype.then[W.Jt]=o,t.on("executor-start",(function(e){e[0]=n(e[0],"resolve-",this,null,!1),e[1]=n(e[1],"resolve-",this,null,!1)})),t.on("executor-err",(function(e,t,r){e[1](r)})),t.on("cb-end",(function(e,r,n){t.emit("propagate",[n,!0],this.nextPromise,!1,!1)})),t.on("propagate",(function(e,r,n){this.getCtx&&!r||(this.getCtx=function(){if(e instanceof Promise)var r=t.context(e);return r&&r.getCtx?r.getCtx():this})}))}(),t}const De={},Ke="setTimeout",Ue="setInterval",Ve="clearTimeout",Ge="-start",Fe=[Ke,"setImmediate",Ue,Ve,"clearImmediate"];function Be(e){const t=function(e){return(e||B.ee).get("timer")}(e);if(De[t.debugId]++)return t;De[t.debugId]=1;var r=(0,W.YM)(t);return r.inPlace(p.gm,Fe.slice(0,2),Ke+"-"),r.inPlace(p.gm,Fe.slice(2,3),Ue+"-"),r.inPlace(p.gm,Fe.slice(3),Ve+"-"),t.on(Ue+Ge,(function(e,t,n){e[0]=r(e[0],"fn-",null,n)})),t.on(Ke+Ge,(function(e,t,n){this.method=n,this.timerDuration=isNaN(e[1])?0:+e[1],e[0]=r(e[0],"fn-",this,n)})),t}const We={};function ze(e){const t=function(e){return(e||B.ee).get("mutation")}(e);if(!p.RI||We[t.debugId])return t;We[t.debugId]=!0;var r=(0,W.YM)(t),n=p.gm.MutationObserver;return n&&(window.MutationObserver=function(e){return this instanceof n?new n(r(e,"fn-")):n.apply(this,arguments)},MutationObserver.prototype=n.prototype),t}const{TZ:qe,d3:Ze,Kp:Ye,$p:Je,wW:Xe,e5:$e,tH:Qe,uP:et,rw:tt,Lc:rt}=je;class nt extends b{static featureName=qe;constructor(e,t=!0){if(super(e,qe,t),!p.RI)return;try{this.removeOnAbort=new AbortController}catch(e){}let r,n=0;const i=this.ee.get("tracer"),o=Le(this.ee),a=Me(this.ee),s=Be(this.ee),c=Z(this.ee),u=this.ee.get("events"),d=ne(this.ee),l=ve(this.ee),f=ze(this.ee);function h(e,t){l.emit("newURL",[""+window.location,t])}function g(){n++,r=window.location.hash,this[et]=(0,N.t)()}function m(){n--,window.location.hash!==r&&h(0,!0);var e=(0,N.t)();this[$e]=~~this[$e]+e-this[et],this[rt]=e}function v(e,t){e.on(t,(function(){this[t]=(0,N.t)()}))}this.ee.on(et,g),a.on(tt,g),o.on(tt,g),this.ee.on(rt,m),a.on(Xe,m),o.on(Xe,m),this.ee.on("fn-err",((...t)={t[2]?.__newrelic?.[e.agentIdentifier]||(0,x.p)("function-err",[...t],void 0,this.featureName,this.ee)})),this.ee.buffer([et,rt,"xhr-resolved"],this.featureName),u.buffer([et],this.featureName),s.buffer(["setTimeout"+Ye,"clearTimeout"+Ze,et],this.featureName),c.buffer([et,"new-xhr","send-xhr"+Ze],this.featureName),d.buffer([Qe+Ze,Qe+"-done",Qe+Je+Ze,Qe+Je+Ye],this.featureName),l.buffer(["newURL"],this.featureName),f.buffer([et],this.featureName),a.buffer(["propagate",tt,Xe,"executor-err","resolve"+Ze],this.featureName),i.buffer([et,"no-"+et],this.featureName),o.buffer(["new-jsonp","cb-start","jsonp-error","jsonp-end"],this.featureName),v(d,Qe+Ze),v(d,Qe+"-done"),v(o,"new-jsonp"),v(o,"jsonp-end"),v(o,"cb-start"),l.on("pushState-end",h),l.on("replaceState-end",h),window.addEventListener("hashchange",h,(0,A.jT)(!0,this.removeOnAbort?.signal)),window.addEventListener("load",h,(0,A.jT)(!0,this.removeOnAbort?.signal)),window.addEventListener("popstate",(function(){h(0,n1)}),(0,A.jT)(!0,this.removeOnAbort?.signal)),this.abortHandler=this.#n,this.importAggregator(e)}#n(){this.removeOnAbort?.abort(),this.abortHandler=void 0}}var it=i(3333);class ot extends b{static featureName=it.TZ;constructor(e,t=!0){super(e,it.TZ,t);const r=[e.init.page_action.enabled,e.init.performance.capture_marks,e.init.performance.capture_measures,e.init.user_actions.enabled,e.init.performance.resources.enabled];if(p.RI&&(e.init.user_actions.enabled&&(it.Zp.forEach((e=(0,A.sp)(e,(e=(0,x.p)("ua",[e],void 0,this.featureName,this.ee)),!0))),it.qN.forEach((e={const t=(0,v.s)((e={(0,x.p)("ua",[e],void 0,this.featureName,this.ee)}),500,{leading:!0});(0,A.sp)(e,t)}))),e.init.performance.resources.enabled&&p.gm.PerformanceObserver?.supportedEntryTypes.includes("resource"))){new PerformanceObserver((e={e.getEntries().forEach((e={(0,x.p)("browserPerformance.resource",[e],void 0,this.featureName,this.ee)}))})).observe({type:"resource",buffered:!0})}r.some((e=e))?this.importAggregator(e):this.deregisterDrain()}}var at=i(993),st=i(3785),ct=i(9414);class ut extends b{static featureName=at.TZ;constructor(e,t=!0){super(e,at.TZ,t);const r=this.ee;(0,ct.J)(r,p.gm.console,"log",{level:"info"}),(0,ct.J)(r,p.gm.console,"error",{level:"error"}),(0,ct.J)(r,p.gm.console,"warn",{level:"warn"}),(0,ct.J)(r,p.gm.console,"info",{level:"info"}),(0,ct.J)(r,p.gm.console,"debug",{level:"debug"}),(0,ct.J)(r,p.gm.console,"trace",{level:"trace"}),this.ee.on("wrap-logger-end",(function([e]){const{level:t,customAttributes:n}=this;(0,st.R)(r,e,n,t)})),this.importAggregator(e)}}new class extends o{constructor(t){super(),p.gm?(this.features={},(0,R.bQ)(this.agentIdentifier,this),this.desiredFeatures=new Set(t.features||[]),this.desiredFeatures.add(w),this.runSoftNavOverSpa=[...this.desiredFeatures].some((e=e.featureName===a.K7.softNav)),(0,d.j)(this,t,t.loaderType||"agent"),this.run()):(0,e.R)(21)}get config(){return{info:this.info,init:this.init,loader_config:this.loader_config,runtime:this.runtime}}run(){try{const t=u(this.agentIdentifier),r=[...this.desiredFeatures];r.sort(((e,t)=a.P3[e.featureName]-a.P3[t.featureName])),r.forEach((r={if(!t[r.featureName]&&r.featureName!==a.K7.pageViewEvent)return;if(this.runSoftNavOverSpa&&r.featureName===a.K7.spa)return;if(!this.runSoftNavOverSpa&&r.featureName===a.K7.softNav)return;const n=function(e){switch(e){case a.K7.ajax:return[a.K7.jserrors];case a.K7.sessionTrace:return[a.K7.ajax,a.K7.pageViewEvent];case a.K7.sessionReplay:return[a.K7.sessionTrace];case a.K7.pageViewTiming:return[a.K7.pageViewEvent];default:return[]}}(r.featureName).filter((e=!(e in this.features)));n.length0&&(0,e.R)(36,{targetFeature:r.featureName,missingDependencies:n}),this.features[r.featureName]=new r(this)}))}catch(t){(0,e.R)(22,t);for(const e in this.features)this.features[e].abortHandler?.();const r=(0,R.Zm)();delete r.initializedAgents[this.agentIdentifier]?.api,delete r.initializedAgents[this.agentIdentifier]?.features,delete this.sharedAggregator;return r.ee.get(this.agentIdentifier).abort(),!1}}}({features:[he,w,S,Se,Oe,O,M,ot,ut,Pe,nt],loaderType:"spa"})})()})();
		}, 5000);
    
    
		window.addEventListener('DOMContentLoaded', function() {
			jQuery(document).ready(function() {
				jQuery('body').on('click', '.oxy-menu-toggle', function() {
					jQuery(this).parent('.oxy-nav-menu').toggleClass('oxy-nav-menu-open');
					jQuery('body').toggleClass('oxy-nav-menu-prevent-overflow');
					jQuery('html').toggleClass('oxy-nav-menu-prevent-overflow');
				});
				var selector = '.oxy-nav-menu-open .menu-item a[href*="#"]';
				jQuery('body').on('click', selector, function(){
					jQuery('.oxy-nav-menu-open').removeClass('oxy-nav-menu-open');
					jQuery('body').removeClass('oxy-nav-menu-prevent-overflow');
					jQuery('html').removeClass('oxy-nav-menu-prevent-overflow');
					jQuery(this).click();
				});
			});
		});

	            window.addEventListener('DOMContentLoaded', function() {   
                
            jQuery(document).ready(oxygen_init_slide_menu);
            function oxygen_init_slide_menu($) {
                
                // check if supports touch, otherwise it's click:
                let touchEvent = 'ontouchstart' in window ? 'click' : 'click';  
                  
                    $('.oxy-slide-menu').each(function(){
                        
                          let slide_menu = $(this);
                          let slide_start = slide_menu.children( '.oxy-slide-menu_inner' ).data( 'start' );
                          let slide_duration = slide_menu.children( '.oxy-slide-menu_inner' ).data( 'duration' );
                          let slideClickArea = '.menu-item-has-children  a  .oxy-slide-menu_dropdown-icon-click-area';
                          let dropdownIcon = slide_menu.children( '.oxy-slide-menu_inner' ).data( 'icon' );
                        
                        
                          slide_menu.find('.menu-item-has-children  a').append('Submenu');
                         
                         // If being hidden as starting position, for use as mobile menu
                          if ( slide_start == 'hidden' ) {

                              let slide_trigger_selector = $( slide_menu.children( '.oxy-slide-menu_inner' ).data( 'trigger-selector' ) );

                              //slide_trigger_selector.click( function( event ) {
                              slide_trigger_selector.on( touchEvent, function(e) {      
                                 slide_menu.slideToggle(slide_duration);
                              } );

                          }
                        
                          if ('enable' === slide_menu.children( '.oxy-slide-menu_inner' ).data( 'currentopen' )) {
                              
                              let currentAncestorButton = slide_menu.find('.current-menu-ancestor').children('a').children('.oxy-slide-menu_dropdown-icon-click-area');
                              
                              currentAncestorButton.attr('aria-expanded', 'true');
                              currentAncestorButton.attr('aria-pressed', 'true');
                              currentAncestorButton.addClass('oxy-slide-menu_open');
                              currentAncestorButton.closest('.current-menu-ancestor').children('.sub-menu').slideDown(0);
                          }
                        
                    });

                 // Sub menu icon being clicked
                 $('.oxy-slide-menu, .oxygen-builder-body').on( touchEvent, '.menu-item-has-children  a  .oxy-slide-menu_dropdown-icon-click-area',  function(e) {  
                        e.stopPropagation();
                        e.preventDefault();
                            oxy_slide_menu_toggle(this);
                        }

                    );
                

                    function oxy_slide_menu_toggle(trigger) {
                                    
                            var durationData = $(trigger).closest('.oxy-slide-menu_inner').data( 'duration' );
                            var othermenus = $(trigger).closest( '.menu-item-has-children' ).siblings('.menu-item-has-children');
                                             othermenus.find( '.sub-menu' ).slideUp( durationData );
                                             othermenus.find( '.oxy-slide-menu_open' ).removeClass( 'oxy-slide-menu_open' );
                                             othermenus.find( '.oxy-slide-menu_open' ).attr('aria-expanded', function (i, attr) {
                                                    return attr == 'true' ? 'false' : 'true'
                                                });
                                            othermenus.find( '.oxy-slide-menu_open' ).attr('aria-pressed', function (i, attr) {
                                                return attr == 'true' ? 'false' : 'true'
                                            });

                            $(trigger).closest('.menu-item-has-children').children('.sub-menu').slideToggle( durationData );

                            $(trigger).attr('aria-expanded', function (i, attr) {
                                return attr == 'true' ? 'false' : 'true'
                            });

                            $(trigger).attr('aria-pressed', function (i, attr) {
                                return attr == 'true' ? 'false' : 'true'
                            });

                            $(trigger).toggleClass('oxy-slide-menu_open');

                        }        
                        
                
                    let selector = '.oxy-slide-menu .menu-item a[href*="#"]';
                    $(selector).on('click', function(event){
                        
                        if ($(event.target).closest('.oxy-slide-menu_dropdown-icon-click-area').length  0) {
                            // toggle icon clicked, no need to trigger it 
                            return;
                        }
                        else if ($(event.target).attr("href") === "#" && $(this).parent().hasClass('menu-item-has-children')) {
                            // prevent browser folllowing link
                            event.preventDefault();
                            // empty href don't lead anywhere, use it as toggle icon click area
                            var hasklinkIcon = $(this).find('.oxy-slide-menu_dropdown-icon-click-area');
                            oxy_slide_menu_toggle(hasklinkIcon);
                            
                        }
                      });

             };
            
        });

                
            window.addEventListener('DOMContentLoaded', function() {
            jQuery(document).ready(oxygen_init_burger);
            function oxygen_init_burger($) {
                
                $('.oxy-burger-trigger').each(function( i, OxyBurgerTrigger ) {
                    
                    let touchEventOption =  $( OxyBurgerTrigger ).children('.hamburger').data('touch');
                    let touchEvent = 'ontouchstart' in window ? touchEventOption : 'click';     
                    
                    // Close hamburger when element clicked 
                    $( OxyBurgerTrigger ).on( touchEvent, function(e) {    
                        
                        e.stopPropagation();

                        // Check user wants animations
                        if ($(this).children( '.hamburger' ).data('animation') !== 'disable') {
                            $(this).children( '.hamburger' ).toggleClass('is-active');
                        }
                        
                    } );
                    
                } );
                
                
                
                // For listening for modals closing to close the hamburger
                var className = 'live';
                var target = document.querySelectorAll(".oxy-modal-backdrop[data-trigger='user_clicks_element']");
                for (var i = 0; i 

    
"use strict";var _createClass=function(){function defineProperties(target,props){for(var i=0;i

var RocketPreloadLinksConfig = {"excludeUris":"\/th(\/wp-admin\/|\/about-us\/|\/search\/|\/faq\/|\/(?:.+\/)?feed(?:\/(?:.+\/?)?)?$|\/(?:.+\/)?embed\/|\/(index.php\/)?(.*)wp-json(\/.*|$))|\/refer\/|\/go\/|\/recommend\/|\/recommends\/","usesTrailingSlash":"1","imageExt":"jpg|jpeg|gif|png|tiff|bmp|webp|avif|pdf|doc|docx|xls|xlsx|php","fileExt":"jpg|jpeg|gif|png|tiff|bmp|webp|avif|pdf|doc|docx|xls|xlsx|php|html|htm","siteUrl":"https:\/\/www.heygoody.com\/th","onHoverDelay":"100","rateThrottle":"3"};


(function() {
"use strict";var r="function"==typeof Symbol&&"symbol"==typeof Symbol.iterator?function(e){return typeof e}:function(e){return e&&"function"==typeof Symbol&&e.constructor===Symbol&&e!==Symbol.prototype?"symbol":typeof e},e=function(){function i(e,t){for(var n=0;ni.config.rateThrottle)return;i.numOnHover++,i._addPrefetchLink(e)},this.config.onHoverDelay);t.addEventListener(n,function e(){t.removeEventListener(n,e,{passive:!0}),null!==r&&(clearTimeout(r),r=null)},{passive:!0})}},{key:"_addPrefetchLink",value:function(i){return this.prefetched.add(i.href),new Promise(function(e,t){var n=document.createElement("link");n.rel="prefetch",n.href=i.href,n.onload=e,n.onerror=t,document.head.appendChild(n)}).catch(function(){})}},{key:"_prepareUrl",value:function(e){if(null===e||"object"!==(void 0===e?"undefined":r(e))||!1 in e||-1===["http:","https:"].indexOf(e.protocol))return null;var t=e.href.substring(0,this.config.siteUrl.length),n=this._getPathname(e.href,t),i={original:e.href,protocol:e.protocol,origin:t,pathname:n,href:t+n};return this._isLinkOk(i)?i:null}},{key:"_getPathname",value:function(e,t){var n=t?e.substring(this.config.siteUrl.length):e;return n.startsWith("/")||(n="/"+n),this._shouldAddTrailingSlash(n)?n+"/":n}},{key:"_shouldAddTrailingSlash",value:function(e){return this.config.usesTrailingSlash&&!e.endsWith("/")&&!this.regex.fileExt.test(e)}},{key:"_isLinkOk",value:function(e){return null!==e&&"object"===(void 0===e?"undefined":r(e))&&(!this.prefetched.has(e.href)&&e.origin===this.config.siteUrl&&-1===e.href.indexOf("?")&&-1===e.href.indexOf("#")&&!this.regex.excludeUris.test(e.href)&&!this.regex.images.test(e.href))}}],[{key:"run",value:function(){"undefined"!=typeof RocketPreloadLinksConfig&&new n(new RocketBrowserCompatibilityChecker({capture:!0,passive:!0}),RocketPreloadLinksConfig).init()}}]),n}();t.run();
}());





            
                document.addEventListener('DOMContentLoaded', function() {

                    // หน่วงเวลาโหลด jQuery (10 วินาที)
                    setTimeout(function() {
                        var script = document.createElement('script');
                        script.src = 'https://www.heygoody.com/th/wp-includes/js/jquery/jquery.min.js';
                        document.body.appendChild(script);
                    }, 10000);

                    // หน่วงเวลาโหลด New Relic (10 วินาที)
                    setTimeout(function() {
                        var script = document.createElement('script');
                        script.src = 'https://js-agent.newrelic.com/nr-spa-1.281.0.min.js';
                        document.body.appendChild(script);
                    }, 10000);
                });
            
                const subMenuData = {
      "product": [
        {
          "title": { "th": "ประกันรถยนต์", "en": "Car Insurance" },
          "imageBeforeTitle": "/th/wp-content/uploads/2025/04/carins-icon-menu-hgd-mobile.webp",
          "children": [
            {
              "title": { "th": "ประกันรถยนต์ทั้งหมด", "en": "All Car Insurance" },
              "url": "/th/autoinsurance/all/"
            },
			{
              "title": { "th": "ประกันรถยนต์ระยะสั้น", "en": "Motor ShortTerm" },
              "url": "/th/autoinsurance/short-term/",
			  "flag": "ใหม่"
            },
            {
              "title": { "th": "ประกันรถยนต์ไฟฟ้า EV", "en": "EV Car Insurance" },
              "url": "/th/autoinsurance/evcar/",
			  "flag": "ใหม่"
            },
            {
              "title": { "th": "ประกันรถยนต์ชั้น 1", "en": "Car Type 1" },
              "url": "/th/autoinsurance/class1/"
            },
            {
              "title": { "th": "ประกันรถยนต์ชั้น 2+,2", "en": "Car Type 2" },
              "url": "/th/autoinsurance/class2plus-2/",
			  "flag": "แนะนำ"
            },
            {
              "title": { "th": "ประกันรถยนต์ชั้น 3+,3", "en": "Car Type 3" },
              "url": "/th/autoinsurance/class3plus-3/"
            },
            {
              "title": { "th": "ประกันรถตู้ส่วนบุคคล", "en": "Personal Truck Insurance" },
              "url": "/checkinsurance/van"
            }
          ]
        },
        {
          "title": { "th": "ประกันเดินทาง", "en": "Travel Insurance" },
          "url": "/th/travelinsurance/",
          "imageBeforeTitle": "/th/wp-content/uploads/2025/04/travel-icon-menu-hgd-mobile.webp"
        },
        {
          "title": { "th": "ประกันบ้าน", "en": "Home Insurance" },
          "url": "/th/homeinsurance/",
          "imageBeforeTitle": "/th/wp-content/uploads/2025/04/home-icon-menu-hgd-mobile.webp"
        },
        {
          "title": { "th": "ประกันโรคมะเร็ง", "en": "Cancer Insurance" },
          "url": "/th/cancer/",
          "imageBeforeTitle": "/th/wp-content/uploads/2025/04/cancer-icon-menu-hgd-mobile.webp"
        },
        {
          "title": { "th": "ประกันโรคร้ายแรง", "en": "Critical Illness Insurance" },
          "url": "/th/critical-illness/",
          "imageBeforeTitle": "/th/wp-content/uploads/2025/04/critical-illness-ins-icon-menu-hgd-mobile.webp"
        },
        {
          "title": { "th": "ประกันชีวิต (ลดหย่อนภาษี)", "en": "Life Insurance (Tax Deduction)" },
		  "imageBeforeTitle": "/th/wp-content/uploads/2025/04/annuity-icon-menu-hgd-mobile.webp",
          "children": [
			{
              "title": { "th": "คำนวณและเปรียบเทียบประกันลดหย่อนภาษี", "en": "" },
              "url": "/th/tax-deduction/"
            },
            {
              "title": { "th": "ประกันสะสมทรัพย์", "en": "" },
              "url": "/th/tax-deduction/savings-insurance/"
            },
            {
              "title": { "th": "ประกันบำนาญ", "en": "" },
              "url": "/th/tax-deduction/annuity-insurance/"
            }
          ]
        }
      ],
      "about": [
        {
          "title": { "th": "รู้จักเรา", "en": "Our History" },
          "url": "/th/about-us/who-we-are/"
        },
        {
          "title": { "th": "ทำไมต้อง heygoody?", "en": "Our Team" },
          "url": "/th/about-us/"
        },
        {
          "title": { "th": "รางวัลความสำเร็จ", "en": "Contact Us" },
          "url": "/th/about-us/awards-and-recognition/"
        }
      ],
      "help": [
        {
          "title": { "th": "ความช่วยเหลือทั้งหมด", "en": "FAQ" },
          "url": "/th/support-info/"
        },
		{
          "title": { "th": "การซื้อประกัน", "en": "" },
          "children": [
            {
              "title": { "th": "การซื้อประกันรถยนต์", "en": "" },
              "url": "/th/support-info/how-to-buy-auto-insurance/"
            },
            {
              "title": { "th": "การซื้อประกันเดินทาง", "en": "" },
              "url": "/th/support-info/how-to-buy-travel-insurance/"
            }
          ]
        },
		{
          "title": { "th": "การจ่ายเงิน", "en": "" },
          "children": [
            {
              "title": { "th": "การจ่ายเงินประกันรถยนต์", "en": "" },
              "url": "/th/support-info/how-to-payment-auto-insurance/"
            },
            {
              "title": { "th": "การจ่ายค่างวดประกันรถยนต์", "en": "" },
              "url": "/th/support-info/how-to-billing/"
            },
            {
              "title": { "th": "การจ่ายเงินประกันเดินทาง", "en": "" },
              "url": "/th/support-info/how-to-payment-travel-insurance/"
            }
          ]
        },
		{
          "title": { "th": "การเคลมประกัน", "en": "" },
          "children": [
            {
              "title": { "th": "การเคลมประกันรถยนต์", "en": "" },
              "url": "/th/support-info/how-to-claim-auto-insurance/"
            },
            {
              "title": { "th": "การเคลมประกันเดินทาง", "en": "" },
              "url": "/th/support-info/how-to-claim-travel-insurance/"
            }
          ]
        },
        {
          "title": { "th": "การใช้งานโปรโมชั่น", "en": "" },
          "url": "/th/support-info/how-to-promotion/"
        }
		,
        {
          "title": { "th": "ค้นหาอู่ซ่อม", "en": "" },
          "url": "/th/support-info/gogogarage/"
        }
		,
        {
          "title": { "th": "คำถามที่พบบ่อย", "en": "" },
          "url": "/th/faq/"
        }
      ],
      "goodytalk": [
        {
          "title": { "th": "บทความ", "en": "Our History" },
          "url": "/th/blogs/"
        },
        {
          "title": { "th": "ข่าวสาร", "en": "Our Team" },
          "url": "/th/news/"
        }
      ]
    };

function createMenuItemLv2(item, menuKey) {
  const aCover = document.createElement("a");
  aCover.classList.add("menu-item-lv2");
  const topText =
      document.querySelector(`#menu-level-1 .top-item[data-menu-key="${menuKey}"]`)
        ?.dataset.menu || menuKey;
    aCover.addEventListener("click", e = {
      patchDataLayerDesktop("lv2", topText, item.title.th);
      e.stopPropagation();
    });
  if (item.url) {
    aCover.href = item.url;
  } else {
    aCover.href = "javascript:void(0);";
    aCover.classList.add("arrow-menu-desktop");
  }

  if (menuKey === "product" && item.imageBeforeTitle) {
    const img = document.createElement("img");
    img.src = item.imageBeforeTitle;
    img.alt = item.title?.th || "icon";
    aCover.appendChild(img);
  }

  if (item.title?.th) {
    const span = document.createElement("span");
    span.textContent = item.title.th;
    aCover.appendChild(span);
  }

  aCover._children = item.children || null;
  return aCover;
}


function createLevel3Content(children, topText) {
  const cover = document.createElement("div");
  const wrap  = document.createElement("div");
  cover.className = "cover-lv3-wrapper";
  wrap.className  = "lv3-wrapper";

  children.forEach(child = {
    const item = document.createElement("div");
    item.classList.add("menu-item-lv3");

    let html = child.title.th;
    if (child.flag) html += child.flag;

    if (child.url) {
      const a = document.createElement("a");
      a.href = child.url;
      a.innerHTML = html;
	  a.addEventListener("click", e = {
        patchDataLayerDesktop("lv3", topText, child.title.th);
        e.stopPropagation();
      }); 
      item.appendChild(a);
    } else item.innerHTML = html;

    wrap.appendChild(item);
  });

  cover.appendChild(wrap);
  return cover;
}

function removeCssLV3() {
  const lv2Covers = document.querySelectorAll(".cover-lv2-wrapper");
  const lv3       = document.getElementById("single-level3");

  lv2Covers.forEach(c =
    lv3.style.display === "block"
      ? c.classList.add("add-new-radius")
      : c.classList.remove("add-new-radius")
  );
}


function initDesktopMenu() {
  const topItems  = document.querySelectorAll("#menu-level-1 .top-item");
  const singleLv3 = document.getElementById("single-level3");

  if (!(window.innerWidth  1200 && topItems.length && singleLv3)) return;

  topItems.forEach(top = {
    const key  = top.dataset.menuKey;
	const topText = top.dataset.menu;
    const data = subMenuData[key];
    if (!data) return;

    const lv2      = document.createElement("div");
    lv2.classList.add("sub-menu-level2", key);
    const coverLv2 = document.createElement("div");
    const wrapLv2  = document.createElement("div");
    coverLv2.className = "cover-lv2-wrapper";
    wrapLv2.className  = `lv2-wrapper ${key}`;
    coverLv2.appendChild(wrapLv2);
    lv2.appendChild(coverLv2);

    const group = document.createElement("div");
    group.classList.add("sub-menu-group", key);
    group.appendChild(lv2);
    top.appendChild(group);

    data.forEach(item = {
      const li = createMenuItemLv2(item, key);
      wrapLv2.appendChild(li);

      li.addEventListener("mouseenter", () = {
        if (item.children?.length) {
          if (!group.contains(singleLv3)) group.appendChild(singleLv3);

          singleLv3.innerHTML = "";
          const lv3Wrap = createLevel3Content(item.children, topText);
          singleLv3.appendChild(lv3Wrap);
          singleLv3.style.display = "block";

          const r = lv2.getBoundingClientRect();
          singleLv3.style.top = r.top + "px";

          wrapLv2.style.height = lv3Wrap.style.height = "";
          requestAnimationFrame(() = {
            const h = Math.max(wrapLv2.offsetHeight, lv3Wrap.offsetHeight);
            wrapLv2.style.height = lv3Wrap.style.height = h + "px";
          });
        } else {
          singleLv3.style.display = "none";
          wrapLv2.style.height = "";
        }
        removeCssLV3();
      });
    });

    top.addEventListener("mouseenter", () = {
      lv2.style.display = "block";
      removeCssLV3();
    });

    const reset = () = {
      group.querySelectorAll(".lv2-wrapper, .lv3-wrapper").forEach(el = (el.style.height = ""));
    };

    function hideAll() {
      lv2.style.display = "none";
      singleLv3.style.display = "none";
      reset();
      removeCssLV3();
    }

    top.addEventListener("mouseleave", e = {
      if (!group.contains(e.relatedTarget)) hideAll();
    });
    lv2.addEventListener("mouseleave", e = {
      if (!group.contains(e.relatedTarget)) hideAll();
    });
  });

  singleLv3.addEventListener("mouseleave", e = {
    const goingToGroup = [...document.querySelectorAll(".sub-menu-group")].some(g = g.contains(e.relatedTarget));
    if (!goingToGroup) {
      singleLv3.style.display = "none";
      document.querySelectorAll(".sub-menu-level2").forEach(lv2 = (lv2.style.display = lv2.style.height = ""));
      removeCssLV3();
    }
  });
}

function isDesktopView() {
  return window.innerWidth  1200;
}

function initMobileMenu() {
  const mobileMenu      = document.getElementById("mobile-menu-container");
  const backDropMenu    = document.getElementById("backDropMenu");
  const singleLv2Mobile = document.getElementById("single-level2-mobile");
  const singleLv3Mobile = document.getElementById("single-level3-mobile");
  const topItems        = document.querySelectorAll("#menu-level-1 .top-item");

  const arrowleft   = "";
  const btImageClose= "";
  const headerLogo  = "";

  const appendHeader = (c, closeHTML, logoHTML) = {
    const h   = document.createElement("div");
    const btn = document.createElement("div");
    const lg  = document.createElement("div");
    h.className   = "header_menu_section";
    btn.className = "close-menu-mobile";
    lg.className  = "logo_menu_section";
    btn.innerHTML = closeHTML;
    lg.innerHTML  = logoHTML;
    h.append(btn, lg);
    c.insertBefore(h, c.firstChild);
  };

  if (!mobileMenu.classList.contains("initialized")) {
    topItems.forEach(it = {
      const clone = it.cloneNode(true);
      clone.querySelector(".sub-menu-level2")?.remove();
      mobileMenu.appendChild(clone);
    });
    mobileMenu.classList.add("initialized");
  }

  document.querySelector(".hamberger-menu").addEventListener("click", () = {
    mobileMenu.classList.add("show");
    backDropMenu.classList.add("open");
  });


  mobileMenu.querySelectorAll(".top-item").forEach(ti = {
    ti.addEventListener("click", function() {
      const key  = this.dataset.menuKey;
      const txt  = this.dataset.menu;
      if (!subMenuData[key]) { 
        const a = this.querySelector("a");
        if (a?.href) window.location.href = a.href;
        return;
      }

      singleLv2Mobile.classList.add("show", key);
      singleLv2Mobile.innerHTML = "";
      appendHeader(singleLv2Mobile, btImageClose, headerLogo);

      const back = document.createElement("div");
      back.className = "back-to-level1 mobile-nav-button";
      back.innerHTML = arrowleft + " กลับไปเมนูหลัก";
      singleLv2Mobile.append(back);

      const tt = document.createElement("div");
      tt.className = "title_menu_mobile";
      tt.innerHTML = txt;
      singleLv2Mobile.append(tt);

      back.addEventListener("click", () = {
        singleLv2Mobile.classList.remove("show");
        setTimeout(() = singleLv2Mobile.classList.remove(key), 320);
      });

      subMenuData[key].forEach((mi, idx) = {
        const di = document.createElement("div");
        di.classList.add("menu_lv_2");
        let html = "";
        if (txt === "ผลิตภัณฑ์ประกัน" && mi.imageBeforeTitle)
          html += ` `;
        html += mi.title.th;
		di.addEventListener(
		  "click",
		  () = patchDataLayerMobile("lv2", txt, mi.title.th)
		);
        if (mi.children?.length) {
          di.dataset.childrenIndex = idx;
          di.classList.add("pre_lv_2");
          di.innerHTML = html;
          di.addEventListener("click", function() {
            const chIdx = this.dataset.childrenIndex;
            const lv3   = subMenuData[key][chIdx].children;

            singleLv3Mobile.classList.add("show", key);
            singleLv3Mobile.innerHTML = "";
            appendHeader(singleLv3Mobile, btImageClose, headerLogo);

            const bb = document.createElement("div");
            bb.className = "back-to-level2 mobile-nav-button";
            bb.innerHTML = arrowleft + " กลับไป" + txt;
            singleLv3Mobile.append(bb);

            const t3 = document.createElement("div");
            t3.className = "title_menu_mobile";
            t3.textContent = subMenuData[key][chIdx].title.th;
            singleLv3Mobile.append(t3);

            bb.addEventListener("click", () = {
              singleLv3Mobile.classList.remove("show");
              setTimeout(() = singleLv3Mobile.classList.remove(key), 320);
            });

            lv3.forEach(it = {
              const d3 = document.createElement("div");
              d3.className = "menu_lv_3";
              let h = it.title.th;
              if (it.flag) h += it.flag;
			  d3.addEventListener(
			   "click",
				() = patchDataLayerMobile("lv3", txt, it.title.th)
				);
              if (it.url) {
                const a = document.createElement("a");
                a.href = it.url;
                a.innerHTML = h;
                a.addEventListener("click", () = {
				  if (isDesktopView()) removeMobileMenus();
				});
                d3.appendChild(a);
              } else d3.innerHTML = h;

              singleLv3Mobile.append(d3);
            });
          });
        } else if (mi.url) {
          di.classList.add("is-link");
          di.innerHTML = html;
          const anchor = document.createElement("a");
          anchor.href = mi.url;
          anchor.appendChild(di);
          anchor.addEventListener("click", () = {
			  if (isDesktopView()) removeMobileMenus();
			});
          singleLv2Mobile.append(anchor);
          return;
        } else di.innerHTML = html;

        singleLv2Mobile.append(di);
      });
    });
  });

  const removeMobileMenus = () = {
    mobileMenu.classList.remove("show");
    singleLv2Mobile.classList.remove("show");
    singleLv3Mobile.classList.remove("show");
    backDropMenu.classList.remove("open");
  };
  backDropMenu.addEventListener("click", e = { if (e.target === backDropMenu) removeMobileMenus(); });
  document.body.addEventListener("click", e = {
    if (e.target.closest(".close-menu-mobile")) removeMobileMenus();
  });
}


document.addEventListener("DOMContentLoaded", () = {
  initDesktopMenu();
  if (window.innerWidth  {
    clearTimeout(rTimer);
    rTimer = setTimeout(() = {
      if (window.innerWidth  1200) { 
		initDataLayer(cat_1 +" | "+ cat_2);
  }
}

function initDataLayer(valData) {
	dataLayer.push({
		event: 'main_menu', 
		menu_label: valData, 
		menu_type: 'header'
	});
}
.top-item {
	position: relative;
	cursor: pointer;
}
.top-item:hover  img {
	transform: rotate(175deg);
    -webkit-transform: rotate(175deg);
}
.top-item  img {
    transition: all 0.3s;
}
.badge_menu.newmenu {
	background-image: linear-gradient(90deg, #0094A7 0%, #81B725 100%);
}
.badge_menu.recommend {
	background-image: linear-gradient(270deg, #9600F2 0%, #FF8400 100%);
}
.badge_menu {
	margin-left: 8px;
	font-size: 12px;
    font-weight: bold;
    font-stretch: normal;
    font-style: normal;
    line-height: normal;
    letter-spacing: normal;
	display: inline-block;
    padding: 1px 8px;
    color: #f1efe3;
    text-align: center;
    white-space: nowrap;
    vertical-align: baseline;
	border-radius: 50rem;
}
.sub-menu-group {
	display: flex;
    top: 0px;
    left: 0;
    position: absolute;
}
.arrow-menu-desktop::after {
content: "";
    background-image: url(/th/wp-content/uploads/2025/04/chevron-right-menu-hgd-mobile.webp);
    background-size: 100%;
    width: 24px;
    height: 24px;
	position: absolute;
    right: 12px;
}
.menu-item-lv2  a:hover,.menu-item-lv3  a:hover {
	color: unset !important;
}
.menu-item-lv2  img {
	width: 32px;
	height: 32px;
	margin-right: 12px;
}
.lv2-wrapper,.lv3-wrapper {
	padding: 8px 8px 8px 16px;
}
.cover-lv2-wrapper {
	position: relative;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 3px 0px 3px 0px rgb(0 0 0 / 15%);
}
.cover-lv3-wrapper {
	position: relative;
    border-top-right-radius: 16px;
	border-bottom-right-radius: 16px;
    overflow: hidden;
	border-left: 1px solid #ddd;
    box-shadow: 3px 0px 3px 0px rgb(0 0 0 / 15%);
}
.cover-lv2-wrapper.add-new-radius {
	border-top-left-radius: 16px;
	border-bottom-left-radius: 16px;
	border-top-right-radius: 0px;
	border-bottom-right-radius: 0px;
}
.sub-menu-level2.product {
	min-width: 297px;
}
.sub-menu-level2.about {
	min-width: 180px;
}
.sub-menu-level2.help {
	min-width: 250px;
}
.sub-menu-level2.goodytalk {
	min-width: 148px;
}
.sub-menu-level2 {
	display: none;
	padding-top: 34px;
	z-index: 1999;
}
.cover-lv2-wrapper,.cover-lv3-wrapper {
	background: #fff;
	transition: height .9s ease;
}

#single-level3 {
	display: none;       
	z-index: 2000;
	margin-left: -6px;
	margin-top: 34px;
	
}
.menu-item-lv3  a {
	width: 100%;
	padding: 16px;
}
.menu-item-lv2 {
	font-weight: bold;
	padding: 16px;
}
.menu-item-lv2,.menu-item-lv3 {
	white-space: nowrap;
	position: relative;
    align-items: center;
    display: flex;
	max-height: 56px;
	color: #1D1D1D;
    text-decoration: unset;
}
.menu-item-lv2:hover,.menu-item-lv3:hover {
	background-color: #cde4ff;
	border-radius: 8px;
	color: #1D1D1D;
}

.hamberger-menu {
	display: none; 
	padding: 8px 8px 8px 6px;
	cursor: pointer;
}
.close-menu-mobile {
	width: 100%;
    height: 36px;
    max-width: 36px;
	display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1;
}
.header_menu_section {
	text-align: center;
    flex-direction: row;
    display: flex;
    gap: 8px;
    align-items: center;
    width: 100%;
    padding-top: 8px;
    padding-bottom: 8px;
    padding-left: 12px;
    padding-right: 12px;
}
.logo_menu_section {
	width: 100%;
    align-items: center;
    margin-left: -42px;
}
.back-to-level1,.back-to-level2 {
	padding: 12px 28px;
    width: 100%;
    margin-top: 18px;
	display: flex;
	align-items: center;
}
.title_menu_mobile {
	padding: 12px 28px;
	margin-top: 8px;
	font-weight: 700;
    font-size: 18px;
	color: #13875D;
}
.menu_lv_2 {
    align-items: center;
    padding-top: 12px;
    padding-bottom: 11px;
    padding-left: 32px;
    padding-right: 40px;
    width: 100%;
    justify-content: flex-start;
    display: flex;
	font-weight: bold;
}
.menu_lv_3 {
    align-items: center;
    padding-top: 8px;
    padding-bottom: 8px;
    padding-left: 30px;
    padding-right: 40px;
    width: 100%;
    justify-content: flex-start;
    display: flex;
	font-weight: normal;
}
.img_icon_menu_mobile {
	width: 32px;
    height: 32px;
    margin-right: 16px;
}
.pre_lv_2:after {
    content: "";
    background-image: url(/th/wp-content/uploads/2025/04/chevron-right-menu-hgd-mobile.webp);
    background-size: 100%;
    width: 24px;
    height: 24px;
    margin-left: auto;
    transition: transform 0.25s ease-in-out;
}
@media (max-width: 1399px) {
	#div_block-593-11 {
		max-width: 1180px;
	}
}
@media (max-width: 1200px) {
	#div_block-593-11 {
		max-width: 100%;
	}
	#menu-level-1 {
		gap: 24px;
	}
}
@media (max-width: 1200px) {
	#div_block-595-11 {
		width: 100%;
	}
	#div_block-594-11 {
		padding: 8px 12px 8px 12px;
	}
	#menu-level-1,
	.sub-menu-level2,
	#single-level3,#div_block-620-11 {
		display: none;
	}

	.hamberger-menu {
		display: block;
	}
	.sub-mobile-menu.show  .top-item {
		display: block;
	}

	#mobile-menu-container,#single-level2-mobile,#single-level3-mobile {
		position: fixed;
		top: 0; 
		left: -100%;
		width: 100%;
		height: 100%;
		overflow-y: auto;
		background: #fff;
		z-index: 9999;
		transition: all 0.4s ease-in-out;
	}
	#mobile-menu-container.show,#single-level2-mobile.show,#single-level3-mobile.show {
		left: 0;
	}
	.mobile-top-item {
		padding: 16px;
		border-bottom: 1px solid #eee;
		font-size: 18px;
		cursor: pointer;
	}
	.mobile-top-item:hover {
		background: #f0f0f0;
	}
}
.sub-mobile-menu {
	left: -100%;
	position: absolute;
}
#backDropMenu.open {
	opacity: 1;
    transition: opacity 0.8s;
	visibility: visible;
}
.sub-mobile-menu::after {
	content: "";
	width: 12px;
	height: 100vh;
	position: absolute;
	right: 0;	
}
.lv2-wrapper::after {
	content: "";
	width: 6px;
	height: 100%;
	position: absolute;
	bottom: 0;
	left: 0;
}
.sub-mobile-menu.product::after,.lv2-wrapper.product::after {
	background-image: linear-gradient(180deg, #03B2C9 0%, #9BDB2E 100%);
}
.sub-mobile-menu.about::after,.lv2-wrapper.about::after {
	background-image: linear-gradient(192deg, #36B38A 59.89%, #088B86 72.26%, #096B65 84.52%);
}
.sub-mobile-menu.help::after,.lv2-wrapper.help::after {
	background-image: linear-gradient(180deg, #FF8400 0%, #9600F2 100%);
}
.sub-mobile-menu.goodytalk::after,.lv2-wrapper.goodytalk::after {
	background-image: linear-gradient(180deg, #71E6E5 0%, #A32BFA 100%);
}
.filter_coupon {
	background: #FFF;
	color: #5c5c5c;
}
.filter_coupon.active-filter {
	background: #13875D;
	color: #f9f9f4;
}
#campaign_CountDown {
	display : none;
}
body {
    font-family: 'notosansthai'!important;
	background-color: #ffffff !important;
	text-size-adjust: none;
  	-webkit-text-size-adjust: none;
  	-ms-text-size-adjust: none;
  	-moz-text-size-adjust: none;
	scroll-behavior: smooth;
}
.slidetext_item * {
    font-family: 'notosansthai' !important;
}

.car_insurance  img {
	border-radius: 8px;
}
@media (min-width: 768px) {
	.pagination div {
		width: 32px;
		height: 4px;
	}
	.button_Arrow_Left_Slide,.button_Arrow_Right_Slide {
		width: 60px;
	}
}

@media (max-width: 766px) {
	.pagination div {
		width: 24px;
		height: 4px;
	}
	.button_Arrow_Left_Slide,.button_Arrow_Right_Slide {
		width: 40px;
	}
}

@media (max-width: 992px) {
	#slideshow  a  picture  img {
		border-radius: 0px !important;
	}
}
.copyText_s2_top {
	cursor: pointer;
}
#slideshow  a  picture  img {
	width: 100%;
    height: auto;
	border-radius: 16px;
}
#div_block-206-2395::-webkit-scrollbar,#div_block-341-2395::-webkit-scrollbar,#div_block-356-2395::-webkit-scrollbar {
    display: none; /* This hides the scrollbar */
}
.slide-text-section {
	background-color: #070816 !important;
}

@keyframes animateBg {
  0% { background-position: 0% 0%; }
  100% { background-position: 100% 0%; }
}
#campaign-section.active{
	background: linear-gradient(90deg, #FA0FB0 0%, #8148E7 50%, #58C2FF 100%);
}

.hgd-countdown-container {
	gap: 16px;
    display: flex;
    justify-content: center;
    align-items: baseline;
    color: #F9F9F4;
}
.hgd-the-colon {
	font-size: 10px;
	font-weight: 700;
	line-height: normal;
	color: #1D1D1D;
}
.hgd-countdown-text {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
}

.hgd-campaign-text {
    font-size: 16px;
    font-weight: 700;
	line-height: normal;
}

.hgd-details-link {
    font-size: 16px;
    color: #F9F9F4;
    text-decoration: underline;
}
.hgd-details-link:hover {
	color: #f7e4ed;
    text-decoration: underline;
}
.hgd-countdown-timer {
    display: flex;
    gap: 4px;
	flex-direction: row;
	align-items: center;
}

.hgd-time-label {
    font-size: 16px;
	font-weight: 700;
	line-height: normal;
}


.hgd-time-box {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 32px;
    height: 32px;
    background-color: black;
    color: white;
    font-size: 14px;
    font-weight: bold;
    border-radius: 4px;
}

@media (max-width: 768px) {
	.hgd-countdown-timer {
		flex-direction: column;
		align-items: start;
		gap: 2px;
	}
	.hgd-countdown-container {
		gap: 8px;
	}
	.hgd-time-box {
		width: 24px;
		height: 24px;
		font-size: 14px;
	}
	.hgd-campaign-text {
		font-size: 14px;
	}
	.hgd-time-label {
		font-size: 14px;
	}
	.hgd-details-link {
    	font-size: 14px;
	}
}
.code_block-411-8 { 
	position: absolute;
    right: 0;
    transform: translateY(-50%);
    top: 50%;
}
.textslide {
	display: inline-block;
    width: max-content;
	height: 100%;
    display: flex;
    align-items: center;
  	animation-timing-function: linear;
  	animation-iteration-count: infinite;
	will-change: transform;
backface-visibility: hidden;
-webkit-backface-visibility: hidden;
-webkit-font-smoothing: antialiased;
-moz-osx-font-smoothing: grayscale;
	-webkit-font-smoothing: subpixel-antialiased;
}
.slidetext_item {
	width: fit-content;
    display: flex;
    align-items: center;
	will-change: transform;
backface-visibility: hidden;
-webkit-backface-visibility: hidden;
-webkit-font-smoothing: antialiased;
-moz-osx-font-smoothing: grayscale;
	-webkit-font-smoothing: subpixel-antialiased;
	-webkit-text-size-adjust: 100%;
}

font {
    pointer-events: none;
}

.pipe_html {
	color: #ffffff;
	font-size: 24px;
}
.emoji-crop  .emoji {
	width: 18px;
}
#code_block-289-2395 {
	background: linear-gradient(270deg, #070816 71.95%, rgba(7, 8, 22, 0.00) 100%);
}
.shelf-deactive {
	background-color: #EBEBEB !important;
	stroke-width: 0.667px;
	stroke: var(--line-line-primary, #DCDCDC);
	filter: drop-shadow(0px 1.5px 8px rgba(0, 0, 0, 0.10));
}
.shelf-active {
	background-color: #13875D !important;
	filter: drop-shadow(0px 1.5px 8px rgba(0, 0, 0, 0.10));
}
.shelf-deactive  svg  path {
	fill: #B8B8B8 !important;
}
.shelf-active  svg  path {
	fill: #F9F9F4 !important;
}
#next-shelf {
	cursor: pointer;
	background-color: #13875D;
	filter: drop-shadow(0px 1.5px 8px rgba(0, 0, 0, 0.10));
}
#prev-shelf {
	cursor: pointer;
	background-color: #EBEBEB;
	stroke-width: 0.667px;
	stroke: var(--line-line-primary, #DCDCDC);
	filter: drop-shadow(0px 1.5px 8px rgba(0, 0, 0, 0.10));
}

}
.category-buttons {
	margin-bottom: 20px;
}
.category-buttons button {
	padding: 8px 15px;
	cursor: pointer;
}
.coupon-left  .title {
    margin: 0;
    font-size: 18px;
	font-style: normal;
	font-weight: 700;
	line-height: normal;
}
.coupon-container {
	display: flex;
    background-color: transparent;
    flex-shrink: 0;
    flex-direction: row;
	position: relative;
    width: 330px;
    border-radius: 8px;
    box-shadow: 6px 2px 6px 0px rgb(0 98 69 / 20%);
    align-items: flex-start;
	height: 158px;
}

.coupon-left {
	-webkit-mask-image: radial-gradient(circle at 0 50%, transparent 15px, #000 16px);
	width: 104px;
	height: 100%;
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	padding: 8px;
	color: #fff;
	text-align: center;
	border-top-left-radius: 8px;
    border-bottom-left-radius: 8px;
}

.car_insurance_ev {
	background: linear-gradient(45deg, #00CE9A, #1164FF);
}

.travel_insurance {
	background: linear-gradient(200deg, #EB009D 1.67%, #FF5ACE 60.58%, #FF7F70 98.31%);
}

.car_insurance {
	background: linear-gradient(21deg, #36B38A 0.44%, #088B86 50.46%, #096B65 100%);
}

.coupon-left img {
	max-width: 56px;
}

.coupon-right {
	display: flex;
	border-top-right-radius: 8px;
    border-bottom-right-radius: 8px;
    background: #FFF;
	flex-direction: column;
	align-items: start;
	height: 100%;
    padding: 6px 8px;
	width: 226px;
}


.coupon-actions {
	margin-top: auto;
	display: flex;
	flex-direction: column;
	width: 100%;
}

.btn-orange {
	background-color: orange;
	border: none;
	cursor: pointer;
	border-radius: 4px;
	background: linear-gradient(90deg, #FA650F 0%, #FD392B 100%);
	color: #fff;
	font-weight: 700;
	width: 72px;
	height: 32px;
	font-size: 14px;
	font-style: normal;
	font-weight: 700;
	line-height: normal;
}

.condition_text {
	text-decoration: underline;
	cursor: pointer;
	font-size: 16px;
	color: #13875D;
	font-weight: 700;
}

.discount_text {
	color: #1D1D1D;
	font-size: 20px;
	font-style: normal;
	font-weight: 700;
	line-height: normal;
}

.minimum_text,.payment_text {
	font-size: 16px;
	font-style: normal;
	font-weight: 400;
	line-height: normal;
}
.minimum_text {
	color: #1D1D1D;
}
.payment_text {
	color: #5C5C5C;
	pointer-events: none;
}
.discount_code {
	color: #FA650F;
	font-size: 16px;
	font-style: normal;
	font-weight: 700;
	line-height: normal;
}
.coupon-section {
	height: 100%;
	width: 100%;
    display: flex;
    gap: 2px;
    padding: 0px 4px;
    border-radius: 4px;
    background: #FFEFE3;
    align-items: center; 
}
@media (max-width: 768px) {
	.discount_text {
		font-size: 16px;
	}
	.minimum_text,.payment_text {
		font-size: 12px;
	}
	.coupon-container {
		width: 272px;
		height: 130px;
	}
	.coupon-left {
		width: 80px;	
	}
	.condition_text {
		font-size: 12px;
	}
	.discount_code {
		font-size: 12px;
	}
	.coupon-section {
		width: 112px;	
	}
	.btn-orange {
		height: 28px;	
	}
	.coupon-right {
		width: 192px;
	}
	.coupon-left  .title {
		font-size: 14px;	
	}
	.coupon-left img {
		max-width: 40px;
	}
	.btn-orange {
		font-size: 12px;	
	}
}

#nextBtnCoupon:disabled,#prevBtnCoupon:disabled {
	background-color: #EBEBEB;
    stroke-width: 0.667px;
    stroke: var(--line-line-primary, #DCDCDC);
    filter: drop-shadow(0px 1.5px 8px rgba(0, 0, 0, 0.10));
}

#prevBtnCoupon:disabled svg path,#nextBtnCoupon:disabled svg path {
    fill: #B8B8B8;
}
#prevBtnCoupon svg path,#nextBtnCoupon svg path {
    fill: #F9F9F4;
}
#prevBtnCoupon,#nextBtnCoupon {
	position: absolute;
    top: 50%;
    transform: translateY(-50%);
	background: transparent;
    border: unset;
	background-color: #13875D;
    border-radius: 50%;
    height: 36px;
    width: 36px;
	padding: 6px;
    filter: drop-shadow(0px 1.5px 8px rgba(0, 0, 0, 0.10));
}
#prevBtnCoupon {
	left: 36px;
}
#nextBtnCoupon {
	right: 36px;
}




#code_block-198-2395 {
	transition: ease .4s;
	cursor: pointer;
}
#code_block-484-8 {
	cursor: pointer;
}
.activeHeightToggle   svg {
	transform: rotate(180deg);
}
.add_blur_af::before {
	content: "";
    display: block;
    position: absolute;
    top: -8px;
    width: 100%;
    height: calc(100% + -14px);
    background-image: linear-gradient(to bottom,rgba(29,29,29,0), #fff 120%);
    transition: opacity .5s;
}
.add_blur_af.active::before {
	display: none;
}
.dot {
	width: 6px;
	height: 4px;
	background-color: gray;
	margin: 0 4px;
	transition: background-color 0.3s ease;
}

.dot.active {
	background-color: #13875D;
}
#nextBtn-review {
	cursor: pointer;
    background-color: #13875D;
    filter: drop-shadow(0px 1.5px 8px rgba(0, 0, 0, 0.10));
}
#nextBtn-review.disabled {
    background-color: #EBEBEB;
}
#prevBtn-review {
	cursor: pointer;
	background-color: #13875D;
	stroke-width: 0.667px;
	stroke: var(--line-line-primary, #DCDCDC);
	filter: drop-shadow(0px 1.5px 8px rgba(0, 0, 0, 0.10));
}
#prevBtn-review.disabled {
	background-color: #EBEBEB;
}
 .product-shelf-section {
        transition: transform 0.5s ease-in-out;
}
.product-shelf-section.active {
	transform: translateX(0);
}

.slide-out-left {
	transform: translateX(-100%);
}
.slide-out-right {
	transform: translateX(100%);
}

.slide-in-left {
	transform: translateX(-100%);
}

.slide-in-right {
	transform: translateX(100%);
}

#code_block-359-2395 {
	cursor: pointer;
}
document.addEventListener('DOMContentLoaded', () = {
  const container = document.querySelector('.scroll-container-award');
  const track     = document.getElementById('custom-scrollbar-award');
  const thumb     = document.getElementById('custom-scrollbar-thumb-award');

  let isDown   = false;
  let startX   = 0;
  let baseLeft = 0;


  const pageX = e = e.touches ? e.touches[0].pageX : e.pageX;

  const updateThumb = () = {
    const maxScroll  = container.scrollWidth - container.clientWidth;
    if (maxScroll  {
    isDown   = true;
    startX   = pageX(e) - container.offsetLeft;
    baseLeft = container.scrollLeft;
    if (e.pointerId != null) container.setPointerCapture?.(e.pointerId);
  };

  const dragMove = e = {
    if (!isDown) return;
    const walk = (pageX(e) - startX - container.offsetLeft) * 1.5;
    container.scrollLeft = baseLeft - walk;
    e.preventDefault(); 
  };

  const dragEnd = () = (isDown = false);


  if ('PointerEvent' in window) {
    container.addEventListener('pointerdown', dragStart);
    container.addEventListener('pointermove', dragMove);
    container.addEventListener('pointerup',   dragEnd);
    container.addEventListener('pointercancel', dragEnd);
  } else {
    container.addEventListener('mousedown', dragStart);
    container.addEventListener('mousemove', dragMove);
    container.addEventListener('mouseup',   dragEnd);
    ['touchstart','touchmove','touchend','touchcancel'].forEach(evt =
      container.addEventListener(evt, evt === 'touchmove' ? dragMove : dragEnd, 
        { passive: evt === 'touchstart' }));
  }

  container.addEventListener('scroll', updateThumb);
  updateThumb(); 
});


const dataLicense = {
  kpp_1: [
    {
      title: "เลขที่ใบอนุญาตประกันวินาศภัย",
      image: "/th/wp-content/uploads/2025/05/image-footer-heygoody-broker-license-hgd.webp",
    },
  ],
  kpp_2: [
    {
      title: "เลขที่ใบอนุญาตเสนอขายประกันภัยผ่านช่องทางอิเล็กทรอนิกส์",
      image: "/th/wp-content/uploads/2025/05/image-footer-heygoody-broker-license-online-hgd.webp",
    },
  ],
};


const EXCLUDE_CLOSE_IDS = ["text_block-201-1080", "text_block-202-1080"];

const License     = document.getElementById("open-license-number");
const areaLicense = document.getElementById("areaLicense");
const titleBox    = document.getElementById("title-license");
const imageBox    = document.getElementById("imageLicense");

function openLicense(key) {
  const item = dataLicense[key]?.[0]; 

  if (item) {
    titleBox.textContent = item.title;    
    imageBox.src  = item.image;          
    imageBox.alt  = item.title;              
  } else {
    titleBox.textContent = "ไม่พบข้อมูลใบอนุญาต";
    imageBox.removeAttribute("src");
    imageBox.alt = "";
  }

  License.classList.add("open");              
  document.body.style.overflow = "hidden";   
}

function closeLicense() {
  License.classList.remove("open");
  document.body.style.overflow = "";
}
  
document.addEventListener("click", (e) = {

  if (!License.classList.contains("open")) return;

  if (areaLicense.contains(e.target)) return;

  const isInExclude = EXCLUDE_CLOSE_IDS.some((id) =
    document.getElementById(id)?.contains(e.target)
  );
  if (isInExclude) return;

  closeLicense();
});
.scroll-container-award::-webkit-scrollbar {
    display: none;
}
@media screen and (min-width: 1601px) {
  .scroll-container-award {
    overflow-x: unset !important;
  }
  #code_block-112-1080 {
    display: none;
  }
}
#open-license-number.open {
	opacity: 1;
    transition: opacity 0.8s;
    visibility: visible;
}
window.lazyLoadOptions=[{elements_selector:"img[data-lazy-src],.rocket-lazyload,iframe[data-lazy-src]",data_src:"lazy-src",data_srcset:"lazy-srcset",data_sizes:"lazy-sizes",class_loading:"lazyloading",class_loaded:"lazyloaded",threshold:300,callback_loaded:function(element){if(element.tagName==="IFRAME"&&element.dataset.rocketLazyload=="fitvidscompatible"){if(element.classList.contains("lazyloaded")){if(typeof window.jQuery!="undefined"){if(jQuery.fn.fitVids){jQuery(element).parent().fitVids()}}}}}},{elements_selector:".rocket-lazyload",data_src:"lazy-src",data_srcset:"lazy-srcset",data_sizes:"lazy-sizes",class_loading:"lazyloading",class_loaded:"lazyloaded",threshold:300,}];window.addEventListener('LazyLoad::Initialized',function(e){var lazyLoadInstance=e.detail.instance;if(window.MutationObserver){var observer=new MutationObserver(function(mutations){var image_count=0;var iframe_count=0;var rocketlazy_count=0;mutations.forEach(function(mutation){for(var i=0;i0||iframe_count0||rocketlazy_count0){lazyLoadInstance.update()}});var b=document.getElementsByTagName("body")[0];var config={childList:!0,subtree:!0};observer.observe(b,config)}},!1)function lazyLoadThumb(e,alt){var t='',a='';t=t.replace('alt=""','alt="'+alt+'"');return t.replace("ID",e)+a}function lazyLoadYoutubeIframe(){var e=document.createElement("iframe"),t="ID?autoplay=1";t+=0===this.parentNode.dataset.query.length?'':'&'+this.parentNode.dataset.query;e.setAttribute("src",t.replace("ID",this.parentNode.dataset.src)),e.setAttribute("frameborder","0"),e.setAttribute("allowfullscreen","1"),e.setAttribute("allow", "accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"),this.parentNode.parentNode.replaceChild(e,this.parentNode)}document.addEventListener("DOMContentLoaded",function(){var e,t,p,a=document.getElementsByClassName("rll-youtube-player");for(t=0;t 



