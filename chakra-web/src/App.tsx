import { useEffect, useRef } from 'react';
import Lenis from 'lenis';
import gsap from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';
import SplitType from 'split-type';
import ThreeScene from './ThreeScene';
import './index.css';

gsap.registerPlugin(ScrollTrigger);

export default function App() {
  const cursorRef   = useRef<HTMLDivElement>(null);
  const ringRef     = useRef<HTMLDivElement>(null);
  const progressRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    /* ── LENIS ── */
    const lenis = new Lenis({
      duration: 1.4,
      easing: (t: number) => Math.min(1, 1.001 - Math.pow(2, -10 * t)),
    });
    lenis.on('scroll', ScrollTrigger.update);
    gsap.ticker.add((time) => { lenis.raf(time * 1000); });
    gsap.ticker.lagSmoothing(0);

    /* ── CURSOR ── */
    let mx = 0, my = 0, rx = 0, ry = 0;
    const onMove = (e: MouseEvent) => {
      mx = e.clientX; 
      my = e.clientY;
      if (cursorRef.current) {
        cursorRef.current.style.left = mx + 'px';
        cursorRef.current.style.top  = my + 'px';
      }
    };
    window.addEventListener('mousemove', onMove);
    
    // Smooth lerp for the ring
    const tickCursor = () => {
      rx += (mx - rx) * 0.15;
      ry += (my - ry) * 0.15;
      if (ringRef.current) {
        ringRef.current.style.left = rx + 'px';
        ringRef.current.style.top  = ry + 'px';
      }
      requestAnimationFrame(tickCursor);
    };
    tickCursor();

    /* ── SCROLL PROGRESS ── */
    gsap.to(progressRef.current, {
      scaleX: 1, ease: 'none',
      scrollTrigger: { trigger: 'body', start: 'top top', end: 'bottom bottom', scrub: true },
    });

    /* ── WHEEL FADE OUT after hero ──
       The canvas wrapper fades to 0 opacity as soon as hero ends.
       After that, every section sits on pure dark background.         */
    gsap.to('#canvas-wrap', {
      opacity: 0,
      ease: 'power2.inOut',
      scrollTrigger: {
        trigger: '#hero',
        start: 'bottom 60%',   // starts fading when bottom of hero reaches 60%
        end:   'bottom top',   // fully gone when hero exits viewport
        scrub: 1,
      },
    });

    /* ── HERO ANIMATIONS ── */
    const tl = gsap.timeline({ delay: 0.5 });
    tl.to('.hero-tag',    { opacity: 1, y: 0, duration: 1,  ease: 'power3.out' })
      .to('.hero-title',  { opacity: 1, duration: 0.01 }, '-=.3');

    const split = new SplitType('.hero-title', { types: 'chars' });
    tl.from(split.chars!, { y: '120%', opacity: 0, duration: 1.2, ease: 'power4.out', stagger: 0.05 }, '-=.5')
      .to('.hero-divider', { opacity: 1, duration: .8, ease: 'power2.out' }, '-=.4')
      .to('.hero-sub',     { opacity: 1, y: 0, duration: .9, ease: 'power3.out' }, '-=.5')
      .to('.hero-meta',    { opacity: 1, duration: .8 }, '-=.4')
      .to('.hero-scroll',  { opacity: 1, duration: .8 }, '-=.3');

    /* Hero itself fades on scroll */
    gsap.to('#hero .hero-content', {
      opacity: 0, y: -40, ease: 'none',
      scrollTrigger: { trigger: '#hero', start: 'center top', end: 'bottom top', scrub: 1 },
    });

    /* ── SECTION TAGS ── */
    gsap.utils.toArray<Element>('.section-tag').forEach(el => {
      gsap.to(el, { opacity: 1, y: 0, duration: .8, ease: 'power3.out',
        scrollTrigger: { trigger: el, start: 'top 85%' },
      });
    });

    /* ── PROBLEM ── */
    document.querySelectorAll('.problem-headline .inner').forEach((el, i) => {
      gsap.from(el, { y: '110%', duration: .9, ease: 'power4.out', delay: i * .12,
        scrollTrigger: { trigger: '.problem-headline', start: 'top 78%' },
      });
    });
    gsap.to('.island-card', { opacity: 1, y: 0, duration: .7, stagger: .15, ease: 'power3.out',
      scrollTrigger: { trigger: '.islands-grid', start: 'top 80%' },
    });
    gsap.to('.hack-item', { opacity: 1, x: 0, duration: .6, stagger: .12, ease: 'power3.out',
      scrollTrigger: { trigger: '.hack-row', start: 'top 82%' },
    });
    gsap.to('.problem-verdict', { opacity: 1, duration: .8,
      scrollTrigger: { trigger: '.problem-verdict', start: 'top 85%' },
    });

    /* ── SOLUTION ── */
    document.querySelectorAll('.solution-headline .inner').forEach((el, i) => {
      gsap.from(el, { y: '110%', duration: 1, ease: 'power4.out', delay: i * .1,
        scrollTrigger: { trigger: '.solution-headline', start: 'top 78%' },
      });
    });
    gsap.to('.claim', { opacity: 1, x: 0, duration: .6, stagger: .12, ease: 'power3.out',
      scrollTrigger: { trigger: '.solution-claims', start: 'top 80%' },
    });
    gsap.to('.flow-step', { opacity: 1, x: 0, duration: .6, stagger: .1, ease: 'power3.out',
      scrollTrigger: { trigger: '.flow-visual', start: 'top 80%' },
    });
    gsap.to('.chain-burst-item', { opacity: 1, y: 0, duration: .5, stagger: .08, ease: 'power3.out',
      scrollTrigger: { trigger: '.chains-burst', start: 'top 82%' },
    });

    /* ── ROADMAP ── */
    document.querySelectorAll('.roadmap-headline .inner').forEach((el, i) => {
      gsap.from(el, { y: '110%', duration: .9, ease: 'power4.out', delay: i * .1,
        scrollTrigger: { trigger: '.roadmap-headline', start: 'top 80%' },
      });
    });
    gsap.to('.phase-card', { opacity: 1, y: 0, duration: .7, stagger: .12, ease: 'power3.out',
      scrollTrigger: { trigger: '.phases', start: 'top 80%' },
    });

    /* ── CTA ── */
    const ctaTl = gsap.timeline({
      scrollTrigger: { trigger: '#cta', start: 'top 72%' },
    });
    ctaTl
      .to('.cta-eyebrow',  { opacity: 1, y: 0, duration: .8, ease: 'power3.out' })
      .to('.cta-title',    { opacity: 1, y: 0, duration: 1,  ease: 'power4.out' }, '-=.4')
      .to('.cta-sub',      { opacity: 1, y: 0, duration: .8, ease: 'power3.out' }, '-=.5')
      .to('.cta-buttons',  { opacity: 1, y: 0, duration: .8, ease: 'power3.out' }, '-=.4')
      .to('.cta-footnote', { opacity: 1,       duration: .8 },                     '-=.2');

    return () => {
      lenis.destroy();
      ScrollTrigger.getAll().forEach(t => t.kill());
      document.removeEventListener('mousemove', onMove);
    };
  }, []);


  return (
    <>
      {/* Cursor */}
      <div id="cursor"      ref={cursorRef} />
      <div id="cursor-ring" ref={ringRef}   />

      {/* Scroll progress */}
      <div id="progress" ref={progressRef} />

      {/* Noise */}
      <div id="noise" />


      {/* 3D — fixed behind everything, fades after hero */}
      <ThreeScene />

      {/* NAV */}
      <nav>
        <div className="nav-logo">CHAKRA</div>
        <ul className="nav-links">
          <li><a href="#problem">Problem</a></li>
          <li><a href="#solution">Solution</a></li>
          <li><a href="#roadmap">Roadmap</a></li>
          <li><a href="#cta" className="nav-cta">Follow Build</a></li>
        </ul>
      </nav>

      {/* ════════════ CONTENT ════════════ */}
      <div id="content">

        {/* ══ HERO ══ */}
        <section id="hero">
          {/* hero-content wrapper so we can fade just the text, not the canvas */}
          <div className="hero-content">
            <p className="hero-tag">Cross-Chain Execution Protocol · Built on Solana</p>
            <h1 className="hero-title">CHAKRA</h1>
            <div className="hero-divider" />
            <p className="hero-sub">
              One Solana program. <strong>Every chain. Atomically.</strong><br />
              No bridges. No wrapped tokens. No trust assumptions.<br />
              Solana becomes the Mainframe of the Internet.
            </p>
            <div className="hero-meta">
              <div className="hero-meta-item">
                <span className="hero-meta-num">1</span>
                <span className="hero-meta-label">Signature</span>
              </div>
              <div className="hero-meta-sep" />
              <div className="hero-meta-item">
                <span className="hero-meta-num">∞</span>
                <span className="hero-meta-label">Chains</span>
              </div>
              <div className="hero-meta-sep" />
              <div className="hero-meta-item">
                <span className="hero-meta-num">0</span>
                <span className="hero-meta-label">Bridges</span>
              </div>
              <div className="hero-meta-sep" />
              <div className="hero-meta-item">
                <span className="hero-meta-num">150ms</span>
                <span className="hero-meta-label">Finality</span>
              </div>
            </div>
            <div className="hero-scroll">
              <div className="scroll-bar" />
              <span>Scroll</span>
            </div>
          </div>
        </section>

        {/* ══ PROBLEM — pure dark, no wheel ══ */}
        <section id="problem">
          <div className="section-inner">
            <div className="section-tag">01 — The Crisis</div>
            <h2 className="problem-headline">
              <span className="line"><span className="inner">The blockchain world</span></span>
              <span className="line"><span className="inner gold">is a collection</span></span>
              <span className="line"><span className="inner red">of broken islands.</span></span>
            </h2>
            <div className="islands-grid">
              {[
                { badge:'badge-btc', label:'Bitcoin',  title:'The Isolated Giant',   desc:'The most valuable asset in the world — trapped. No smart contracts. No composability. No connection to the broader economy. $900B sitting idle.' },
                { badge:'badge-eth', label:'Ethereum', title:'The Fragmented Hub',   desc:'Powerful but slow. High fees. Sprawling L2 ecosystem that\'s more fractured than unified. Everyone "connected" but nobody truly interoperable.' },
                { badge:'badge-sol', label:'Solana',   title:'The Fastest Island',   desc:'The fastest financial engine on earth — but still an island. Can\'t reach Bitcoin. Can\'t control Ethereum. Can\'t be the global rail it\'s destined to be.' },
              ].map((c, i) => (
                <div className="island-card" key={i}>
                  <span className={`island-badge ${c.badge}`}>{c.label}</span>
                  <div className="island-title">{c.title}</div>
                  <div className="island-desc">{c.desc}</div>
                  <div className="island-status">Isolated ✕</div>
                </div>
              ))}
            </div>
            <div className="hack-row">
              {[
                { amount:'$625M', name:'Ronin Bridge — Hacked'    },
                { amount:'$320M', name:'Wormhole — Exploited'     },
                { amount:'$190M', name:'Nomad — Drained'          },
              ].map((h, i) => (
                <div className="hack-item" key={i}>
                  <span className="hack-amount">{h.amount}</span>
                  <span className="hack-name">{h.name}</span>
                </div>
              ))}
            </div>
            <p className="problem-verdict">
              In 2022 alone, bridges lost <strong>over $2 billion</strong> to hacks.<br />
              The current solution is <strong>the problem.</strong>
            </p>
          </div>
        </section>

        <div className="h-line" />

        {/* ══ SOLUTION ══ */}
        <section id="solution">
          <div className="section-inner">
            <div className="section-tag">02 — The Command Layer</div>
            <h2 className="solution-headline">
              <span className="line"><span className="inner">Solana becomes</span></span>
              <span className="line gold"><span className="inner">the Mainframe.</span></span>
              <span className="line"><span className="inner">Every chain,</span></span>
              <span className="line"><span className="inner">a peripheral.</span></span>
            </h2>
            <div className="solution-grid">
              <div className="solution-left">
                <div className="solution-claims">
                  {[
                    { title:'No bridges. Ever.',            desc:'CHAKRA\'s Distributed Key Generation (DKG) splits the master key across a network of Sentinel Nodes. No single point of failure. No honeypot to hack.' },
                    { title:'One signature controls everything.', desc:'Sign a single transaction on Solana. CHAKRA\'s TSS relayer executes the corresponding action on any target chain — Bitcoin, Ethereum, Base, Sui — natively.' },
                    { title:'Atomic or nothing.',           desc:'Time-locked escrow on Solana ensures atomicity. If the target chain doesn\'t confirm within the window, the action rolls back. Zero partial execution.' },
                    { title:'Quantum-resistant from day one.', desc:'Built with Lattice-Based Post-Quantum Cryptography. When ECDSA dies in 2026+, CHAKRA is already safe. Every other bridge will be vulnerable.' },
                  ].map((c, i) => (
                    <div className="claim" key={i}>
                      <div className="claim-dot" />
                      <div>
                        <div className="claim-title">{c.title}</div>
                        <div className="claim-desc">{c.desc}</div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
              <div className="solution-right">
                <div className="flow-visual">
                  <div className="flow-step active">
                    <span className="flow-badge fb-sol">SOL</span>
                    <div>
                      <div className="flow-text-label">You sign on Solana</div>
                      <div className="flow-text-sub">single transaction · one wallet</div>
                    </div>
                  </div>
                  <div className="flow-arrow">↓ CHAKRA intercepts</div>
                  <div className="flow-step active">
                    <span className="flow-badge fb-chakra">CHAKRA</span>
                    <div>
                      <div className="flow-text-label">DKG nodes combine key shards</div>
                      <div className="flow-text-sub">threshold signature · trustless</div>
                    </div>
                  </div>
                  <div className="flow-arrow">↓ atomic execution</div>
                  <div className="flow-step">
                    <span className="flow-badge fb-all">ALL</span>
                    <div>
                      <div className="flow-text-label">Target chains execute</div>
                      <div className="flow-text-sub">native · no wrapping · confirmed</div>
                    </div>
                  </div>
                  <div className="chains-burst" style={{ marginTop:'2px' }}>
                    {[
                      { dot:'cb-btc',  name:'Bitcoin',  status:'✓ executed' },
                      { dot:'cb-eth',  name:'Ethereum', status:'✓ executed' },
                      { dot:'cb-base', name:'Base',     status:'✓ executed' },
                      { dot:'cb-sui',  name:'Sui',      status:'✓ executed' },
                    ].map((c, i) => (
                      <div className="chain-burst-item" key={i}>
                        <div className={`cbdot ${c.dot}`} />
                        <span className="cb-name">{c.name}</span>
                        <span className="cb-status">{c.status}</span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>

        <div className="h-line" />

        {/* ══ ROADMAP ══ */}
        <section id="roadmap">
          <div className="section-inner">
            <div className="section-tag">03 — The Path to Legacy</div>
            <h2 className="roadmap-headline">
              <span className="line"><span className="inner">Four phases.</span></span>
              <span className="line gold"><span className="inner">One outcome.</span></span>
            </h2>
            <div className="phases">
              {[
                { num:'Phase 01', period:'Months 1–2', chip:'chip-live',   chipLabel:'Building Now', title:'The First Turn',        desc:'Build the Controller logic in Rust/Anchor on Solana Devnet. Prove a Solana signature can trigger remote execution on an EVM chain without any bridge.',    goal:'Proof-of-concept: Solana → Base cross-chain execution' },
                { num:'Phase 02', period:'Months 3–4', chip:'chip-soon',   chipLabel:'Next',         title:'The Universal Vault',   desc:'Launch the CHAKRA Dashboard. A single UI where you manage BTC, ETH, and SOL using only your Solana wallet. No bridges. No wrapping.',                goal:'One wallet. Every chain. One interface.' },
                { num:'Phase 03', period:'Months 5–6', chip:'chip-future', chipLabel:'Upcoming',     title:'The Developer SDK',     desc:'Open the CHAKRA API. Any developer on Solana can build Universal Apps that tap the liquidity of every chain simultaneously.',                        goal:'Grant target: $100k+ infrastructure funding' },
                { num:'Phase 04', period:'Month 7+',   chip:'chip-future', chipLabel:'The Legacy',   title:'The Sentinel Network',  desc:'Full decentralization. 100,000 laptops and phones as Sentinel Nodes. CHAKRA becomes the Standard Protocol for internet-wide state management.',    goal:'Solana = Mainframe of the Internet' },
              ].map((p, i) => (
                <div className="phase-card" key={i}>
                  <span className={`phase-chip ${p.chip}`}>{p.chipLabel}</span>
                  <div className="phase-num">{p.num}</div>
                  <div className="phase-title">{p.title}</div>
                  <div className="phase-period">{p.period}</div>
                  <div className="phase-desc">{p.desc}</div>
                  <div className="phase-goal">{p.goal}</div>
                </div>
              ))}
            </div>
          </div>
        </section>

        <div className="h-line" />

        {/* ══ CTA ══ */}
        <section id="cta">
          <div className="cta-inner">
            <p className="cta-eyebrow">The Infinite Wheel · Built in Public</p>
            <h2 className="cta-title">
              It doesn't<br /><span className="gold">stop.</span>
            </h2>
            <p className="cta-sub">
              <strong>1 developer. 1 laptop. $0 funding.</strong><br />
              Building the infrastructure the multichain world has been waiting for.<br />
              Follow the build. We're just getting started.
            </p>
            <div className="cta-buttons">
              <a href="https://x.com/chandana_mndrpu" target="_blank" rel="noreferrer" className="btn-gold">Follow on X</a>
              <a href="https://github.com/ChandanaMandarapu"  target="_blank" rel="noreferrer" className="btn-outline">GitHub</a>
            </div>
            <p className="cta-footnote">The wheel turns. It doesn't ask for permission.</p>
          </div>
        </section>

      </div>{/* #content */}

      <footer>
        <div className="footer-logo">CHAKRA</div>
        <div className="footer-copy">© 2025 — Building in public · Solana Devnet</div>
      </footer>
    </>
  );
}