import React, { useRef, useMemo } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { Float, MeshDistortMaterial, Sphere, Torus, Stars } from '@react-three/drei';
import * as THREE from 'three';

const InfiniteWheel = () => {
  const meshRef = useRef<THREE.Mesh>(null);
  const outerRef = useRef<THREE.Group>(null);

  useFrame((state) => {
    const t = state.clock.getElapsedTime();
    if (meshRef.current) {
      meshRef.current.rotation.x = t * 0.2;
      meshRef.current.rotation.y = t * 0.5;
    }
    if (outerRef.current) {
      outerRef.current.rotation.z = -t * 0.1;
    }
  });

  return (
    <group ref={outerRef}>
      {/* The Central Core */}
      <Float speed={2} rotationIntensity={1} floatIntensity={2}>
        <Sphere args={[1, 64, 64]} ref={meshRef}>
          <MeshDistortMaterial
            color="#ffc107"
            speed={3}
            distort={0.4}
            radius={1}
            emissive="#ffc107"
            emissiveIntensity={2}
            metalness={0.9}
            roughness={0.1}
          />
        </Sphere>
      </Float>

      {/* The Outer Chakra Ring */}
      <Torus args={[2.5, 0.02, 16, 100]} rotation={[Math.PI / 2, 0, 0]}>
        <meshStandardMaterial color="#00f2ff" emissive="#00f2ff" emissiveIntensity={5} />
      </Torus>
      
      {/* Decorative Orbits */}
      {[...Array(3)].map((_, i) => (
        <Torus 
          key={i}
          args={[2.5 + (i * 0.5), 0.01, 16, 100]} 
          rotation={[Math.PI / (2 + i), Math.PI / (4 * i), 0]}
        >
          <meshStandardMaterial color="#ffffff" opacity={0.2} transparent />
        </Torus>
      ))}
    </group>
  );
};

const ThreeScene = () => {
  return (
    <div className="canvas-container">
      <Canvas camera={{ position: [0, 0, 8], fov: 45 }}>
        <color attach="background" args={['#050505']} />
        <ambientLight intensity={0.5} />
        <pointLight position={[10, 10, 10]} intensity={2} color="#ffc107" />
        <pointLight position={[-10, -10, -10]} intensity={1} color="#00f2ff" />
        <Stars radius={100} depth={50} count={5000} factor={4} saturation={0} fade speed={1} />
        
        <InfiniteWheel />
        
        {/* Post Processing Effects could be added here for more OOMPH */}
      </Canvas>
    </div>
  );
};

export default ThreeScene;
