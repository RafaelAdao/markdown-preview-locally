(function () {
  var currentPath = (document.querySelector('meta[name="current-path"]') || {}).content || '';
  var banner = document.getElementById('reconnect-banner');
  var ws;

  // ── Content update (single point so copy buttons are always injected) ────────

  function setContent(html) {
    var el = document.querySelector('.markdown-body');
    el.innerHTML = html;
    addCopyButtons(el);
    renderMermaid(el);
  }

  // ── Copy buttons ─────────────────────────────────────────────────────────────

  function addCopyButtons(container) {
    container.querySelectorAll('pre').forEach(function (pre) {
      if (pre.querySelector('.copy-btn')) return;
      pre.style.position = 'relative';

      var btn = document.createElement('button');
      btn.className = 'copy-btn';
      btn.textContent = 'Copy';
      btn.setAttribute('aria-label', 'Copy code');

      btn.addEventListener('click', function (e) {
        e.preventDefault();
        var code = pre.querySelector('code');
        var text = (code || pre).innerText.replace(/\n$/, '');
        copyToClipboard(text, btn);
      });

      pre.appendChild(btn);
    });
  }

  function copyToClipboard(text, btn) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text).then(function () { showCopied(btn); });
    } else {
      var ta = document.createElement('textarea');
      ta.value = text;
      ta.style.cssText = 'position:fixed;opacity:0;pointer-events:none';
      document.body.appendChild(ta);
      ta.select();
      try { document.execCommand('copy'); showCopied(btn); } catch (_) {}
      document.body.removeChild(ta);
    }
  }

  function showCopied(btn) {
    btn.textContent = 'Copied!';
    btn.classList.add('copied');
    setTimeout(function () {
      btn.textContent = 'Copy';
      btn.classList.remove('copied');
    }, 2000);
  }

  // ── Mermaid diagrams ─────────────────────────────────────────────────────────
  // mermaid.run() reads .innerHTML and sees HTML-escaped text (e.g. "--&gt;" instead
  // of "-->"), which breaks parsing. We use mermaid.render(id, text) instead,
  // passing node.textContent so the browser's HTML decoding happens first.

  var mermaidSeq = 0;

  function renderMermaid(container) {
    if (!window.mermaid || !container) return;
    mermaid.initialize({ startOnLoad: false, theme: currentTheme() === 'dark' ? 'dark' : 'default' });
    var nodes = Array.from(container.querySelectorAll('.mermaid:not([data-processed="true"])'));
    nodes.forEach(function (node) {
      var source = node.textContent.trim();
      node.setAttribute('data-processed', 'true');
      mermaid.render('mmd-' + (++mermaidSeq), source)
        .then(function (result) {
          node.innerHTML = result.svg;
          if (result.bindFunctions) result.bindFunctions(node);
        })
        .catch(function () {});
    });
  }

  // ── Theme ────────────────────────────────────────────────────────────────────

  function currentTheme() {
    return document.documentElement.dataset.theme || 'light';
  }

  function applyTheme(theme) {
    document.documentElement.dataset.theme = theme;
    var isDark = theme === 'dark';
    document.getElementById('css-light').media = isDark ? 'not all' : 'all';
    document.getElementById('css-dark').media  = isDark ? 'all' : 'not all';
    localStorage.setItem('mdpreview-theme', theme);
  }

  window.toggleTheme = function () {
    var next = currentTheme() === 'dark' ? 'light' : 'dark';
    applyTheme(next);
    if (currentPath) {
      fetchContent(currentPath, next, setContent);
    }
  };

  // ── Live reload ──────────────────────────────────────────────────────────────

  function connect() {
    ws = new WebSocket('ws://' + location.host + '/ws');

    ws.onopen = function () { banner.style.display = 'none'; };

    ws.onmessage = function (e) {
      try {
        var msg = JSON.parse(e.data);
        if (msg.type === 'reload' && msg.path === currentPath) {
          fetchContent(currentPath, currentTheme(), setContent);
        }
      } catch (_) {}
    };

    ws.onclose = function () {
      banner.style.display = 'block';
      setTimeout(connect, 1500);
    };
  }

  // ── File navigation ──────────────────────────────────────────────────────────

  function fetchContent(path, theme, cb) {
    fetch('/render?path=' + encodeURIComponent(path) + '&theme=' + encodeURIComponent(theme))
      .then(function (r) { return r.ok ? r.text() : null; })
      .then(function (html) { if (html != null) cb(html); });
  }

  window.loadFile = function (el) {
    var path = el.getAttribute('data-path');
    fetchContent(path, currentTheme(), function (html) {
      setContent(html);
      currentPath = path;

      document.querySelectorAll('#sidebar a[data-path]').forEach(function (a) {
        a.classList.toggle('active', a.getAttribute('data-path') === path);
      });

      document.title = path.split('/').pop() + ' — mdpreview';

      var content = document.getElementById('content');
      if (content) content.scrollTop = 0;
    });
  };

  // ── Directory toggle ─────────────────────────────────────────────────────────

  window.toggleDir = function (label) {
    var li = label.closest('.tree-dir');
    if (li) li.classList.toggle('collapsed');
  };

  // ── Init ─────────────────────────────────────────────────────────────────────

  var initTheme = currentTheme();
  applyTheme(initTheme);

  // Server always renders initial content in light mode; re-fetch if dark saved.
  if (initTheme === 'dark' && currentPath) {
    fetchContent(currentPath, 'dark', setContent);
  } else {
    var body = document.querySelector('.markdown-body');
    addCopyButtons(body);
    renderMermaid(body);
  }

  connect();
})();
