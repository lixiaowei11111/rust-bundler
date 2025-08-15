import { greet } from './utils.js';
import config from './config.json';

console.log('Hello from Rust Bundler!');
greet('World');
console.log('Config:', config);