/* ── RUST JOB PREP — Shared Navigation ──
   Drop <div id="nav-root"></div> + <script src="/nav.js"></script>
   anywhere in <body>. The script injects the full navbar and
   hides the scrollbar on html/body automatically.
   Active day is detected from window.location.pathname.
*/

(function () {
  /* ── Scrollbar hide ── */
  const style = document.createElement('style');
  style.textContent = `
    html, body {
      scrollbar-width: none;
    }
    html::-webkit-scrollbar,
    body::-webkit-scrollbar {
      display: none;
    }

    /* ── Shared navbar styles ── */
    #rust-nav-topbar {
      background: #111111;
      border-bottom: 1px solid #333333;
      height: 48px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 32px;
      position: sticky;
      top: 0;
      z-index: 200;
      font-family: 'JetBrains Mono', monospace;
    }
    #rust-nav-topbar .nb-left {
      display: flex;
      align-items: center;
      gap: 14px;
      font-size: 11px;
      color: #888888;
    }
    #rust-nav-topbar .nb-dot {
      width: 8px; height: 8px;
      background: #ff4500;
      flex-shrink: 0;
    }
    #rust-nav-topbar .nb-path span { color: #f5f5f5; }
    #rust-nav-topbar .nb-right {
      font-size: 11px;
      color: #4a4a4a;
    }

    #rust-nav-days {
      background: #1a1a1a;
      border-bottom: 1px solid #242424;
      overflow-x: auto;
      scrollbar-width: none;
    }
    #rust-nav-days::-webkit-scrollbar { display: none; }
    #rust-nav-days .nd-inner {
      display: flex;
      min-width: max-content;
    }
    #rust-nav-days .nd-label {
      font-family: 'JetBrains Mono', monospace;
      font-size: 10px;
      color: #4a4a4a;
      letter-spacing: 2px;
      text-transform: uppercase;
      padding: 0 20px;
      border-right: 1px solid #242424;
      display: flex;
      align-items: center;
      flex-shrink: 0;
    }
    #rust-nav-days a {
      font-family: 'JetBrains Mono', monospace;
      font-size: 10px;
      font-weight: 600;
      color: #4a4a4a;
      text-decoration: none;
      padding: 10px 16px;
      border-right: 1px solid #242424;
      white-space: nowrap;
      display: flex;
      flex-direction: column;
      gap: 2px;
      transition: background 0.12s, color 0.12s;
      position: relative;
    }
    #rust-nav-days a:hover { background: #242424; color: #f5f5f5; }
    #rust-nav-days a.nd-active { color: #f5f5f5; background: #242424; }
    #rust-nav-days a.nd-active::after {
      content: '';
      position: absolute;
      bottom: 0; left: 0; right: 0;
      height: 2px;
      background: #ff4500;
    }
    #rust-nav-days a .nd-num { color: inherit; letter-spacing: 1px; }
    #rust-nav-days a .nd-topic {
      font-size: 9px;
      color: #4a4a4a;
      font-weight: 400;
    }
    #rust-nav-days a.nd-active .nd-topic,
    #rust-nav-days a:hover .nd-topic { color: #888888; }
  `;
  document.head.appendChild(style);

  /* ── Nav data ── */
  const days = [
    { href: '/',      num: 'HOME',   topic: 'eval'         },
    { href: '/day01', num: 'DAY 01', topic: 'ownership'    },
    { href: '/day02', num: 'DAY 02', topic: 'lifetimes'    },
    { href: '/day03', num: 'DAY 03', topic: 'traits'       },
    { href: '/day04', num: 'DAY 04', topic: 'error handling'},
    { href: '/day05', num: 'DAY 05', topic: 'enums'        },
    { href: '/day06', num: 'DAY 06', topic: 'iterators'    },
    { href: '/day07', num: 'DAY 07', topic: 'async/await'  },
    { href: '/day08', num: 'DAY 08', topic: 'concurrency'  },
    { href: '/day09', num: 'DAY 09', topic: 'smart ptrs'   },
    { href: '/day10', num: 'DAY 10', topic: 'macros'       },
    { href: '/day11', num: 'DAY 11', topic: 'unsafe rust'  },
    { href: '/day12', num: 'DAY 12', topic: 'proc macros'  },
    { href: '/day13', num: 'DAY 13', topic: 'ffi'          },
    { href: '/day14', num: 'DAY 14', topic: 'mem layout'   },
    { href: '/day15', num: 'DAY 15', topic: 'testing'      },
  ];

  /* Detect current page — normalize trailing slash */
  const path = window.location.pathname.replace(/\/$/, '') || '/';

  /* Detect which day label to show in topbar */
  const current = days.find(d => {
    const dPath = d.href === '/' ? '' : d.href;
    return path === dPath || path === d.href;
  });
  const pageLabel = current
    ? (current.num === 'HOME' ? 'PHASE 01 · 15 DAYS' : `PHASE 01 · ${current.num}`)
    : 'PHASE 01 · 15 DAYS';

  /* Detect file path for topbar breadcrumb */
  const fileLabel = path === '' || path === '/'
    ? 'index.html'
    : path.replace('/', '') + '/index.html';

  /* ── Build HTML ── */
  const topbar = `
    <div id="rust-nav-topbar">
      <div class="nb-left">
        <div class="nb-dot"></div>
        <span class="nb-path">rust-job-prep / <span>phase-01</span> / <span>${fileLabel}</span></span>
      </div>
      <div class="nb-right">${pageLabel}</div>
    </div>
  `;

  const links = days.map(d => {
    const dPath = d.href === '/' ? '' : d.href;
    const isActive = path === dPath || path === d.href;
    return `
      <a href="${d.href}" class="${isActive ? 'nd-active' : ''}">
        <span class="nd-num">${d.num}</span>
        <span class="nd-topic">${d.topic}</span>
      </a>
    `;
  }).join('');

  const dayNav = `
    <nav id="rust-nav-days" aria-label="Day Navigation">
      <div class="nd-inner">
        <span class="nd-label">// PHASE 01</span>
        ${links}
      </div>
    </nav>
  `;

  /* ── Inject ── */
  const root = document.getElementById('nav-root');
  if (root) {
    root.innerHTML = topbar + dayNav;
  } else {
    /* Fallback: prepend to body */
    document.body.insertAdjacentHTML('afterbegin', topbar + dayNav);
  }
})();