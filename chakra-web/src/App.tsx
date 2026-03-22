import React, { useEffect, useRef } from 'react';
import Lenis from 'lenis';
import gsap from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';
import ThreeScene from './ThreeScene';
import './index.css';

gsap.registerPlugin(ScrollTrigger);

function App() {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const lenis = new Lenis();

    lenis.on('scroll', ScrollTrigger.update);

    gsap.ticker.add((time) => {
      lenis.raf(time * 1000);
    });

    gsap.ticker.lagSmoothing(0);

    // Initial Animations
    const ctx = gsap.context(() => {
      // Title Animation
      gsap.from('.title', {
        y: 100,
        opacity: 0,
        duration: 1.5,
        ease: 'power4.out',
        stagger: 0.2
      });

      // Scroll Trigger Animations
      gsap.to('.hero-section', {
        scrollTrigger: {
          trigger: '.hero-section',
          start: 'top top',
          end: 'bottom top',
          scrub: true,
        },
        opacity: 0,
        scale: 0.8
      });

    }, containerRef);

    return () => {
      lenis.destroy();
      ctx.revert();
    };
  }, []);

  return (
    <div ref={containerRef} className="app-container">
      <div className="noise" />
      <ThreeScene />

      <main className="content-layer">
        <section className="hero-section">
          <h1 className="title">CHAKRA</h1>
          <p className="subtitle">
            The <span className="highlight">Universal Mainframe</span> of the Multichain.
            One Signature. No Bridges. Pure Power.
          </p>
        </section>

        <section className="problem-section">
          <h2 className="title">The Fracture</h2>
          <p className="subtitle">
            Blockchain is a world of islands. Isolated. Complex. Slow. 
            Bridges are failing. Wrappers are hacking. 
          </p>
        </section>

        <section className="solution-section">
          <h2 className="title">The Command</h2>
          <p className="subtitle">
            Solana as the absolute center. Own and control any account on any chain, 
            natively, atomically, and instantly.
          </p>
        </section>

        <section className="status-section">
          <h2 className="title">Phase 1</h2>
          <p className="subtitle">
            Building on an 8GB laptop. 4 years of grit. 
            Transforming the ledger into the Mainframe.
          </p>
        </section>

        <section className="footer-section">
          <h2 className="title">Join the Wheel</h2>
          <div className="subtitle" style={{ display: 'flex', gap: '2rem' }}>
            <a href="#" style={{ color: 'white', textDecoration: 'none' }}>Twitter</a>
            <a href="#" style={{ color: 'white', textDecoration: 'none' }}>GitHub</a>
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;
