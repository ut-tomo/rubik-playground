import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

// Global variables
let cube;
let scene, camera, renderer, controls;
let cubeGroup;
let wasmModule;

// Corner positions and color definitions
const CORNER_POSITIONS = [
    { pos: [-1, 1, 1] },   // 0: UFL
    { pos: [1, 1, 1] },    // 1: UFR
    { pos: [1, 1, -1] },   // 2: UBR
    { pos: [-1, 1, -1] },  // 3: UBL
    { pos: [-1, -1, 1] },  // 4: DFL
    { pos: [1, -1, 1] },   // 5: DFR
    { pos: [1, -1, -1] },  // 6: DBR
    { pos: [-1, -1, -1] }, // 7: DBL
];

const CORNER_BASE_COLORS = [
    [0, 2, 4],  // 0: UFL (黄・青・赤)
    [0, 4, 3],  // 1: UFR (黄・赤・緑)
    [0, 3, 5],  // 2: UBR (黄・緑・橙)
    [0, 5, 2],  // 3: UBL (黄・橙・青)
    [1, 4, 2],  // 4: DFL (白・赤・青)
    [1, 3, 4],  // 5: DFR (白・緑・赤)
    [1, 5, 3],  // 6: DBR (白・橙・緑)
    [1, 2, 5]   // 7: DBL (白・青・橙)
];

const CORNER_POSITION_INFO = [
    { yFace: 'top', side1: 'left', side2: 'front' },
    { yFace: 'top', side1: 'front', side2: 'right' },
    { yFace: 'top', side1: 'right', side2: 'back' },
    { yFace: 'top', side1: 'back', side2: 'left' },
    { yFace: 'bottom', side1: 'front', side2: 'left' },
    { yFace: 'bottom', side1: 'right', side2: 'front' },
    { yFace: 'bottom', side1: 'back', side2: 'right' },
    { yFace: 'bottom', side1: 'left', side2: 'back' }
];

const EDGE_POSITIONS = [
    { pos: [0, 1, 1], faces: [0, 4] },
    { pos: [1, 1, 0], faces: [0, 3] },
    { pos: [0, 1, -1], faces: [0, 5] },
    { pos: [-1, 1, 0], faces: [0, 2] },
    { pos: [-1, 0, 1], faces: [4, 2] },
    { pos: [1, 0, 1], faces: [4, 3] },
    { pos: [1, 0, -1], faces: [5, 3] },
    { pos: [-1, 0, -1], faces: [5, 2] },
    { pos: [0, -1, 1], faces: [1, 4] },
    { pos: [1, -1, 0], faces: [1, 3] },
    { pos: [0, -1, -1], faces: [1, 5] },
    { pos: [-1, -1, 0], faces: [1, 2] },
];

function updateProgress(percent, status) {
    document.getElementById('progress-bar').style.width = percent + '%';
    document.getElementById('loading-status').textContent = status;
}

function getCornerColors(cornerIndex, state) {
    const perm = state.corner_perm[cornerIndex];
    const ori = state.corner_ori[cornerIndex];
    const baseColors = CORNER_BASE_COLORS[perm];
    
    let rotatedColors;
    if (ori === 0) {
        rotatedColors = [baseColors[0], baseColors[1], baseColors[2]];
    } else if (ori === 1) {
        rotatedColors = [baseColors[1], baseColors[2], baseColors[0]];
    } else {
        rotatedColors = [baseColors[2], baseColors[0], baseColors[1]];
    }
    
    const posInfo = CORNER_POSITION_INFO[cornerIndex];
    const result = {
        top: -1, bottom: -1, left: -1,
        right: -1, front: -1, back: -1
    };
    
    result[posInfo.yFace] = rotatedColors[0];
    result[posInfo.side1] = rotatedColors[1];
    result[posInfo.side2] = rotatedColors[2];
    
    return result;
}

function getEdgeColors(edgeIndex, state) {
    const perm = state.edge_perm[edgeIndex];
    const ori = state.edge_ori[edgeIndex];
    const originalFaces = EDGE_POSITIONS[perm].faces;
    
    const [color1, color2] = ori === 1 ? 
        [originalFaces[1], originalFaces[0]] : 
        [originalFaces[0], originalFaces[1]];
    
    const result = {
        right: -1, left: -1, top: -1,
        bottom: -1, front: -1, back: -1
    };
    
    const edgeMapping = [
        ['top', 'front'], ['top', 'right'], ['top', 'back'], ['top', 'left'],
        ['front', 'left'], ['front', 'right'], ['back', 'right'], ['back', 'left'],
        ['bottom', 'front'], ['bottom', 'right'], ['bottom', 'back'], ['bottom', 'left']
    ];
    
    if (edgeIndex < edgeMapping.length) {
        const [face1, face2] = edgeMapping[edgeIndex];
        result[face1] = color1;
        result[face2] = color2;
    }
    
    return result;
}

function createCubeVisualization(state) {
    if (cubeGroup) {
        scene.remove(cubeGroup);
        cubeGroup.traverse((child) => {
            if (child.geometry) child.geometry.dispose();
            if (child.material) {
                if (Array.isArray(child.material)) {
                    child.material.forEach(mat => mat.dispose());
                } else {
                    child.material.dispose();
                }
            }
        });
    }
    
    cubeGroup = new THREE.Group();
    
    const cubeSize = 0.9;
    const gap = 0.05;
    const colorMap = {
        0: 0xffff00, // Yellow
        1: 0xffffff, // White
        2: 0x2130E7, // Blue
        3: 0x00ff00, // Green
        4: 0xff0000, // Red
        5: 0xffac51, // Orange
    };
    
    // Create corners
    for (let i = 0; i < 8; i++) {
        const [x, y, z] = CORNER_POSITIONS[i].pos;
        const colors = getCornerColors(i, state);
        
        const geometry = new THREE.BoxGeometry(cubeSize, cubeSize, cubeSize);
        const materials = [
            new THREE.MeshStandardMaterial({ color: colors.right >= 0 ? colorMap[colors.right] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: colors.left >= 0 ? colorMap[colors.left] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: colors.top >= 0 ? colorMap[colors.top] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: colors.bottom >= 0 ? colorMap[colors.bottom] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: colors.front >= 0 ? colorMap[colors.front] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: colors.back >= 0 ? colorMap[colors.back] : 0x000000 }),
        ];
        
        const cubelet = new THREE.Mesh(geometry, materials);
        cubelet.position.set(x * (cubeSize + gap), y * (cubeSize + gap), z * (cubeSize + gap));
        
        const edges = new THREE.EdgesGeometry(geometry);
        const line = new THREE.LineSegments(edges, new THREE.LineBasicMaterial({ color: 0x000000, linewidth: 2 }));
        cubelet.add(line);
        cubeGroup.add(cubelet);
    }
    
    // Create edges
    for (let i = 0; i < 12; i++) {
        const [x, y, z] = EDGE_POSITIONS[i].pos;
        const colors = getEdgeColors(i, state);
        
        const geometry = new THREE.BoxGeometry(cubeSize, cubeSize, cubeSize);
        const materials = [
            new THREE.MeshStandardMaterial({ color: colors.right >= 0 ? colorMap[colors.right] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: colors.left >= 0 ? colorMap[colors.left] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: colors.top >= 0 ? colorMap[colors.top] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: colors.bottom >= 0 ? colorMap[colors.bottom] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: colors.front >= 0 ? colorMap[colors.front] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: colors.back >= 0 ? colorMap[colors.back] : 0x000000 }),
        ];
        
        const cubelet = new THREE.Mesh(geometry, materials);
        cubelet.position.set(x * (cubeSize + gap), y * (cubeSize + gap), z * (cubeSize + gap));
        
        const edges = new THREE.EdgesGeometry(geometry);
        const line = new THREE.LineSegments(edges, new THREE.LineBasicMaterial({ color: 0x000000, linewidth: 2 }));
        cubelet.add(line);
        cubeGroup.add(cubelet);
    }
    
    // Create centers
    const centers = [
        { pos: [1, 0, 0], color: 3 },
        { pos: [-1, 0, 0], color: 2 },
        { pos: [0, 1, 0], color: 0 },
        { pos: [0, -1, 0], color: 1 },
        { pos: [0, 0, 1], color: 4 },
        { pos: [0, 0, -1], color: 5 },
    ];
    
    for (const center of centers) {
        const [x, y, z] = center.pos;
        const geometry = new THREE.BoxGeometry(cubeSize, cubeSize, cubeSize);
        const materials = [
            new THREE.MeshStandardMaterial({ color: x === 1 ? colorMap[center.color] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: x === -1 ? colorMap[center.color] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: y === 1 ? colorMap[center.color] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: y === -1 ? colorMap[center.color] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: z === 1 ? colorMap[center.color] : 0x000000 }),
            new THREE.MeshStandardMaterial({ color: z === -1 ? colorMap[center.color] : 0x000000 }),
        ];
        
        const cubelet = new THREE.Mesh(geometry, materials);
        cubelet.position.set(x * (cubeSize + gap), y * (cubeSize + gap), z * (cubeSize + gap));
        
        const edges = new THREE.EdgesGeometry(geometry);
        const line = new THREE.LineSegments(edges, new THREE.LineBasicMaterial({ color: 0x000000, linewidth: 2 }));
        cubelet.add(line);
        cubeGroup.add(cubelet);
    }
    
    scene.add(cubeGroup);
}

function updateVisualization() {
    const state = cube.getState();
    createCubeVisualization(state);
    
    const isSolved = cube.isSolved();
    const statusEl = document.getElementById('status');
    const solvedStatus = document.getElementById('solved-status');
    
    if (isSolved) {
        statusEl.classList.add('solved');
        solvedStatus.textContent = 'Solved ✓';
    } else {
        statusEl.classList.remove('solved');
        solvedStatus.textContent = 'Scrambled';
    }
    
    document.getElementById('state-info').innerHTML = `
        <strong>State:</strong><br>
        <small>
        corner_perm: [${state.corner_perm.join(', ')}]<br>
        corner_ori: [${state.corner_ori.join(', ')}]<br>
        edge_perm: [${state.edge_perm.join(', ')}]<br>
        edge_ori: [${state.edge_ori.join(', ')}]
        </small>
    `;
}

function setupScene() {
    const container = document.getElementById('canvas-container');
    
    scene = new THREE.Scene();
    // Darker background for better contrast
    scene.background = new THREE.Color(0x0b0b12);

    camera = new THREE.PerspectiveCamera(50, 1, 0.1, 1000);
    camera.position.set(5, 5, 5);
    camera.lookAt(0, 0, 0);

    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(100, 100);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.05;

    const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
    scene.add(ambientLight);

    const directionalLight1 = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight1.position.set(5, 10, 5);
    scene.add(directionalLight1);

    const directionalLight2 = new THREE.DirectionalLight(0xffffff, 0.4);
    directionalLight2.position.set(-5, -5, -5);
    scene.add(directionalLight2);

    window.addEventListener('resize', () => {
        const container = document.getElementById('canvas-container');
        const aspect = container.clientWidth / container.clientHeight;
        camera.aspect = aspect;
        camera.updateProjectionMatrix();
        renderer.setSize(container.clientWidth, container.clientHeight);
    });
}

function animate() {
    requestAnimationFrame(animate);
    controls.update();
    renderer.render(scene, camera);
}

let animationStarted = false;
function startAnimation() {
    if (!animationStarted) {
        animationStarted = true;
        animate();
    }
}

function initApp() {
    cube = new wasmModule.WasmCube();
    window.cube = cube;
    window.wasmModule = wasmModule;

    setupScene();
    updateVisualization();
    startAnimation();
}

// Global functions for HTML buttons
window.applyMove = function(moveStr) {
    try {
        const oldRotation = cubeGroup.rotation.y;
        cube.applyMove(moveStr);
        updateVisualization();
        
        const rotationAmount = 0.2;
        cubeGroup.rotation.y = oldRotation + rotationAmount;
        
        const startRotation = cubeGroup.rotation.y;
        const targetRotation = oldRotation;
        const duration = 200;
        const startTime = performance.now();
        
        function animateRotation() {
            const elapsed = performance.now() - startTime;
            const progress = Math.min(elapsed / duration, 1);
            const eased = 1 - Math.pow(1 - progress, 3);
            
            cubeGroup.rotation.y = startRotation + (targetRotation - startRotation) * eased;
            
            if (progress < 1) {
                requestAnimationFrame(animateRotation);
            }
        }
        animateRotation();
    } catch (e) {
        console.error('Error applying move:', e);
        alert('Error: ' + e);
    }
};

window.applyAlgorithm = function() {
    const alg = document.getElementById('algorithm-input').value;
    if (!alg) return;
    
    try {
        cube.applyAlgorithm(alg);
        updateVisualization();
    } catch (e) {
        console.error('Error applying algorithm:', e);
        alert('Error: ' + e);
    }
};

window.showInverse = function() {
    const alg = document.getElementById('algorithm-input').value;
    if (!alg) return;
    
    try {
        const inv = wasmModule.invertAlgorithm(alg);
        document.getElementById('algorithm-input').value = inv;
    } catch (e) {
        console.error('Error calculating inverse:', e);
        alert('Error: ' + e);
    }
};

window.resetCube = function() {
    cube = new wasmModule.WasmCube();
    window.cube = cube;
    updateVisualization();
};

window.applyCommutator = function() {
    const a = document.getElementById('alg-a').value;
    const b = document.getElementById('alg-b').value;
    if (!a || !b) {
        alert('Please enter both algorithms');
        return;
    }
    
    try {
        let result = wasmModule.commutator(a, b);

        result = result.replace(/([UDLRFB])p/g, "$1'");
        document.getElementById('algorithm-input').value = result;
        cube.applyAlgorithm(result);
        updateVisualization();
    } catch (e) {
        console.error('Error applying commutator:', e);
        alert('Error: ' + e);
    }
};

window.applyConjugate = function() {
    const a = document.getElementById('alg-a').value;
    const b = document.getElementById('alg-b').value;
    if (!a || !b) {
        alert('Please enter both algorithms');
        return;
    }
    
    try {
        let result = wasmModule.conjugate(a, b);

        result = result.replace(/([UDLRFB])p/g, "$1'");
        document.getElementById('algorithm-input').value = result;
        cube.applyAlgorithm(result);
        updateVisualization();
    } catch (e) {
        console.error('Error applying conjugate:', e);
        alert('Error: ' + e);
    }
};

// Main initialization
export async function main() {
    try {
        const startTime = performance.now();
        
        updateProgress(10, 'Loading WASM module...');
        const wasmStart = performance.now();
        wasmModule = await import('./pkg/cube_wasm.js');
        console.log(`WASM module loaded in ${(performance.now() - wasmStart).toFixed(0)}ms`);
        
        updateProgress(40, 'Initializing WASM...');
        const initStart = performance.now();
        await wasmModule.default();
        console.log(`WASM initialized in ${(performance.now() - initStart).toFixed(0)}ms`);
        
        updateProgress(60, 'Setting up scene...');
        const sceneStart = performance.now();
        
        updateProgress(80, 'Creating visualization...');
        initApp();
        console.log(`Scene setup in ${(performance.now() - sceneStart).toFixed(0)}ms`);
        
        updateProgress(100, 'Ready!');
        console.log(`Total initialization time: ${(performance.now() - startTime).toFixed(0)}ms`);
        
        document.getElementById('loading').style.display = 'none';
        document.getElementById('app').style.display = 'grid';
        
        setTimeout(() => {
            const container = document.getElementById('canvas-container');
            const width = container.clientWidth;
            const height = container.clientHeight;
            
            if (width > 0 && height > 0) {
                camera.aspect = width / height;
                camera.updateProjectionMatrix();
                renderer.setSize(width, height);
            }
        }, 50);
        
    } catch (error) {
        console.error('Error during initialization:', error);
        document.getElementById('loading').innerHTML = `
            <h2>❌ Error Loading</h2>
            <p>${error.message}</p>
        `;
    }
}
