import { useRef, useMemo } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { Stars } from '@react-three/drei';
import * as THREE from 'three';

const Mandala = () => {
  const masterRef = useRef<THREE.Group>(null);
  const ring1Ref  = useRef<THREE.Group>(null);
  const ring2Ref  = useRef<THREE.Group>(null);
  const ring3Ref  = useRef<THREE.Group>(null);
  const particlesRef = useRef<THREE.Points>(null);

  const GOLD     = new THREE.Color('#C9A227');
  const GOLD_DIM = new THREE.Color('#5a4310');
  const WHITE    = new THREE.Color('#ffffff');

  const particles = useMemo(() => {
    const count = 1200;
    const pos = new Float32Array(count * 3);
    const col = new Float32Array(count * 3);
    for (let i = 0; i < count; i++) {
      const ring = Math.floor(Math.random() * 5);
      const r = 4 + ring * 2.5 + Math.random() * 1.5;
      const theta = Math.random() * Math.PI * 2;
      const phi = (Math.random() - 0.5) * 0.25;
      pos[i*3]   = r * Math.cos(theta) * Math.cos(phi);
      pos[i*3+1] = r * Math.sin(phi) * 2;
      pos[i*3+2] = r * Math.sin(theta) * Math.cos(phi);
      const t = Math.random();
      col[i*3]   = GOLD_DIM.r + (GOLD.r - GOLD_DIM.r) * t;
      col[i*3+1] = GOLD_DIM.g + (GOLD.g - GOLD_DIM.g) * t;
      col[i*3+2] = GOLD_DIM.b + (GOLD.b - GOLD_DIM.b) * t;
    }
    return { pos, col };
  }, []);

  const chains = useMemo(() => [
    { color: '#f7931a', angle: 0 },
    { color: '#627eea', angle: Math.PI * 0.5 },
    { color: '#0052ff', angle: Math.PI },
    { color: '#4da2ff', angle: Math.PI * 1.5 },
  ], []);

  const spokeAngles = useMemo(() =>
    Array.from({ length: 12 }, (_, i) => (i / 12) * Math.PI * 2), []);

  useFrame((state, delta) => {
    // Making it react to mouse/scroll for that "alive" feeling
    const speed = 1 + state.mouse.y * 0.4;
    if (masterRef.current)    masterRef.current.rotation.z    += delta * 0.04 * speed;
    if (ring1Ref.current)     ring1Ref.current.rotation.z     -= delta * 0.08 * speed;
    if (ring2Ref.current)     ring2Ref.current.rotation.z     += delta * 0.05 * speed;
    if (ring3Ref.current)     ring3Ref.current.rotation.z     -= delta * 0.025 * speed;
    if (particlesRef.current) particlesRef.current.rotation.y += delta * 0.008;
  });

  return (
    <group>
      <points ref={particlesRef}>
        <bufferGeometry>
          <bufferAttribute attach="attributes-position" args={[particles.pos, 3]} />
          <bufferAttribute attach="attributes-color"    args={[particles.col, 3]} />
        </bufferGeometry>
        <pointsMaterial size={0.018} vertexColors transparent opacity={0.7} sizeAttenuation />
      </points>

      <group ref={masterRef}>
        {spokeAngles.map((angle, i) => {
          const len = i % 3 === 0 ? 3.2 : i % 3 === 1 ? 2.6 : 2.0;
          return (
            <mesh key={i} position={[Math.cos(angle)*len/2, Math.sin(angle)*len/2, 0]} rotation={[0,0,angle]}>
              <boxGeometry args={[len, i%3===0 ? 0.014 : 0.008, 0.008]} />
              <meshStandardMaterial color={GOLD} emissive={GOLD} emissiveIntensity={i%3===0?0.6:0.3} transparent opacity={i%3===0?0.8:0.45} />
            </mesh>
          );
        })}
        <mesh>
          <torusGeometry args={[1.0, 0.016, 16, 128]} />
          <meshStandardMaterial color={WHITE} emissive={WHITE} emissiveIntensity={0.9} transparent opacity={0.6} />
        </mesh>
        <mesh>
          <torusGeometry args={[2.0, 0.022, 16, 128]} />
          <meshStandardMaterial color={GOLD} emissive={GOLD} emissiveIntensity={0.7} transparent opacity={0.75} />
        </mesh>
        <mesh>
          <torusGeometry args={[3.2, 0.014, 16, 128]} />
          <meshStandardMaterial color={GOLD} emissive={GOLD} emissiveIntensity={0.4} transparent opacity={0.4} />
        </mesh>
        {Array.from({length:8},(_,i)=>{
          const a=(i/8)*Math.PI*2;
          return (
            <mesh key={i} position={[Math.cos(a)*2.0, Math.sin(a)*2.0, 0]}>
              <octahedronGeometry args={[0.1,0]} />
              <meshStandardMaterial color={WHITE} emissive={WHITE} emissiveIntensity={2} />
            </mesh>
          );
        })}
      </group>

      <group ref={ring1Ref}>
        {Array.from({length:24},(_,i)=>{
          const a=(i/24)*Math.PI*2;
          return (
            <mesh key={i} position={[Math.cos(a)*4.2, Math.sin(a)*4.2, 0]}>
              <sphereGeometry args={[i%3===0?0.05:0.025,8,8]} />
              <meshStandardMaterial color={GOLD} emissive={GOLD} emissiveIntensity={i%3===0?1.5:0.5} transparent opacity={i%3===0?1:0.4} />
            </mesh>
          );
        })}
      </group>

      <group ref={ring2Ref}>
        {Array.from({length:6},(_,i)=>{
          const a=(i/6)*Math.PI*2;
          return (
            <mesh key={i} position={[Math.cos(a)*5.4, Math.sin(a)*5.4, 0]} rotation={[0,0,a+Math.PI/2]}>
              <coneGeometry args={[0.08,0.2,3]} />
              <meshStandardMaterial color={GOLD} emissive={GOLD} emissiveIntensity={1} transparent opacity={0.7} />
            </mesh>
          );
        })}
        <mesh>
          <torusGeometry args={[5.4, 0.01, 8, 128]} />
          <meshStandardMaterial color={GOLD} emissive={GOLD} emissiveIntensity={0.3} transparent opacity={0.2} />
        </mesh>
      </group>

      <group ref={ring3Ref}>
        {chains.map((chain, i) => {
          const col  = new THREE.Color(chain.color);
          const dist = 7.2;
          const x    = Math.cos(chain.angle) * dist;
          const y    = Math.sin(chain.angle) * dist;
          return (
            <group key={i}>
              <mesh position={[x*0.5, y*0.5, 0]} rotation={[0,0,chain.angle]}>
                <boxGeometry args={[dist-3.4, 0.007, 0.007]} />
                <meshStandardMaterial color={col} emissive={col} emissiveIntensity={0.6} transparent opacity={0.35} />
              </mesh>
              <mesh position={[x,y,0]} rotation={[0,0,chain.angle]}>
                <torusGeometry args={[0.35,0.04,12,48]} />
                <meshStandardMaterial color={col} emissive={col} emissiveIntensity={1.2} metalness={0.4} roughness={0.3} />
              </mesh>
              <mesh position={[x,y,0]}>
                <sphereGeometry args={[0.16,16,16]} />
                <meshStandardMaterial color={col} emissive={col} emissiveIntensity={2} />
              </mesh>
              <mesh position={[x,y,0]}>
                <torusGeometry args={[0.55,0.007,8,48]} />
                <meshStandardMaterial color={col} emissive={col} emissiveIntensity={0.5} transparent opacity={0.3} />
              </mesh>
            </group>
          );
        })}
      </group>

      <mesh>
        <sphereGeometry args={[0.22,32,32]} />
        <meshStandardMaterial color={GOLD} emissive={GOLD} emissiveIntensity={4} metalness={1} roughness={0} />
      </mesh>
      <mesh>
        <torusGeometry args={[0.45,0.01,12,64]} />
        <meshStandardMaterial color={WHITE} emissive={WHITE} emissiveIntensity={1.5} transparent opacity={0.5} />
      </mesh>
    </group>
  );
};

const ThreeScene = () => (
  <div id="canvas-wrap">
    <Canvas camera={{ position:[0,0,12], fov:48 }} gl={{ antialias:true, alpha:true }} dpr={[1,2]}>
      <color attach="background" args={['#020202']} />
      <fog attach="fog" args={['#020202',20,50]} />
      <ambientLight intensity={0.2} />
      <pointLight position={[6,6,6]}   intensity={4} color="#C9A227" />
      <pointLight position={[-6,-6,4]} intensity={2} color="#ffffff" />
      <pointLight position={[0,0,-10]} intensity={1} color="#C9A227" />
      <Stars radius={90} depth={50} count={4000} factor={3} saturation={0} fade speed={0.4} />
      <Mandala />
    </Canvas>
  </div>
);

export default ThreeScene;