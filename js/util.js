/* jshint esversion: 6, browser: true, devel: true */
/* global VISIBLE_HEIGHT */

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
 * TO GL X
 * Converts an x-coordinate in the browser window
 * to an x-coordinate usable in the WebGL matrix
 * space.
 ************************************************/
const toGLX = (clientX) => (clientX - (width() - height()) / 2) * VISIBLE_HEIGHT / height();

/************************************************
 * TO GL Y
 * Converts an y-coordinate in the browser window
 * to an y-coordinate usable in the WebGL matrix
 * space.
 ************************************************/
const toGLY = (clientY) => (clientY / height()) * VISIBLE_HEIGHT;

/************************************************
 * TO BROWSER X
 * Converts an x-coordinate in the WebGL matrix
 * space to an x-coordinate usable in the browser
 * window.
 ************************************************/
const toBrowserX = (glX) => (glX * height() / VISIBLE_HEIGHT) + ((width() - height()) / 2);

/************************************************
 * TO BROWSER Y
 * Converts an y-coordinate in the WebGL matrix
 * space to an y-coordinate usable in the browser
 * window.
 ************************************************/
const toBrowserY = (glY) => glY * height() / VISIBLE_HEIGHT;

const isMobile = () => {
   return (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent));
};
