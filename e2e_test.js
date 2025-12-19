const puppeteer = require('puppeteer');

(async () => {
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('http://localhost:3000');
  
  // Simulamos escribir en el input 
  await page.type('#q', '¿Qué es Green Computing?');
  await page.click('button');
  
  // Esperamos a ver si aparece la respuesta
  await page.waitForSelector('#res');
  const texto = await page.$eval('#res', el => el.innerText);
  
  console.log(texto.includes('transmisión') ? '❌ Falló' : '✅ E2E Pasó');
  await browser.close();
})();