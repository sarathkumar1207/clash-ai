const button = document.querySelector('#analyzeButton');
const input = document.querySelector('#tagInput');
const result = document.querySelector('#lookupResult');
const impact = document.querySelector('#impactScore');

button?.addEventListener('click', () => {
  const tag = String(input?.value || '').trim().toUpperCase();
  const legalTag = tag.startsWith('#') ? tag : `#${tag}`;
  const score = Math.max(64, Math.min(98, 70 + legalTag.length * 3));
  if (result) {
    result.innerHTML = `<b>${legalTag} analysis queued</b><span>Official API lookup, upgrade coach, clan context, and screenshot review are separated by service boundary.</span>`;
  }
  if (impact) impact.textContent = String(score);
});
