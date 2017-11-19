/* jshint esversion: 6, browser: true, devel: true */

const intToRGB = (rgb) => {
   // 0xrrggbb
   let red = (rgb >> 16) / 255;
   let green = ((rgb >> 8) & 0xff) / 255;
   let blue = (rgb & 0xff) / 255;
   return {
      r: red,
      g: green,
      b: blue
   };
};

const width = () => window.innerWidth;
const height = () => window.innerHeight;

/************************************************
 * TO SCREEN X
 * Converts an x-coordinate in the browser window
 * to an x-coordinate usable in the WebGL matrix
 * space.
 ************************************************/
const toScreenX = (clientX) => (clientX - (width() - height()) / 2) * 1000 / height();

/************************************************
 * TO SCREEN Y
 * Converts an y-coordinate in the browser window
 * to an y-coordinate usable in the WebGL matrix
 * space.
 ************************************************/
const toScreenY = (clientY) => (clientY / height()) * 1000;

const isMobile = () => {
   return (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent));
};
