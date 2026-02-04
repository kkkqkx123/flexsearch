# FlexSearch v0.8.2 - Project Overview

FlexSearch is a next-generation full-text search library for Browser and Node.js environments. It performs queries up to 1,000,000 times faster compared to other libraries while providing powerful search capabilities like multi-field search (document search), phonetic transformations, partial matching, tag-search, result highlighting, and suggestions.

## Language
Always use Chinese in code comments and docs.

## Project Architecture

The project is organized into the following main directories:
- `src/` - Source code files with modular architecture
- `dist/` - Distribution builds for different use cases
- `example/` - Example implementations for various platforms
- `doc/` - Documentation files
- `test/` - Test suite
- `task/` - Build and automation scripts

### Key Source Components
- `index.js` - Main index implementation
- `document.js` - Multi-field document search functionality
- `worker.js` - Web worker support for background processing
- `encoder.js` - Text encoding and transformation utilities
- `resolver.js` - Complex query resolution
- `charset.js` - Character set handling
- `async.js` - Asynchronous processing support
- `db/` - Persistent storage adapters

## Building and Running

### Prerequisites
- Node.js environment

### Build Commands
The project uses npm scripts for building different distributions:

- `npm run build` - Build main bundle and debug version
- `npm run build:all` - Build all distribution variants
- `npm run build:bundle` - Build full-featured bundle
- `npm run build:compact` - Build compact version (no workers)
- `npm run build:light` - Build light version (basic features only)
- `npm run build:es5` - Build ES5 compatible version
- `npm run build:module` - Build ES module versions
- `npm run build:lang` - Build language packs
- `npm run build:db` - Build database connectors

### Testing
- `npm run test:all` - Run all tests

## Key Features

### Core Functionality
1. **Index Types**:
   - `Index` - Flat high-performance index storing id-content pairs
   - `Worker` - Background processing via dedicated worker threads
   - `Document` - Multi-field index for complex JSON documents

2. **Search Capabilities**:
   - Partial matching with various tokenizers
   - Phonetic/phonetic transformations (fuzzy search)
   - Context-aware search
   - Multi-field document search
   - Tag search
   - Result highlighting
   - Suggestions for fuzzy matching

3. **Performance Optimizations**:
   - Multiple tokenizer options (strict, forward, reverse, full)
   - Various encoder presets for different languages
   - Auto-balanced caching by popularity
   - Async non-blocking runtime balancer
   - Worker-based parallel processing

4. **Persistence**:
   - In-memory (default)
   - IndexedDB (Browser)
   - Redis, SQLite, PostgreSQL, MongoDB, ClickHouse

### Distribution Variants
- `bundle` - All features included
- `compact` - Most features except workers
- `light` - Basic features only
- `es5` - ES5 compatibility
- Module variants for ES modules
- Debug versions for development

## Development Conventions

### Code Structure
- Modular architecture with separate files for different components
- Conditional compilation used for feature inclusion/exclusion
- Type definitions provided in `index.d.ts`

### Configuration
- Feature flags in build scripts control what gets compiled
- Multiple preset configurations available (memory, performance, match, score, default)
- Extensive options for customization

### Best Practices
- Use numeric IDs for better performance
- Choose appropriate tokenizer based on use case
- Select encoder preset based on language requirements
- Use async methods for large datasets to prevent blocking
- Leverage caching for frequently accessed queries

## Supported Platforms
- Browser (all modern browsers)
- Node.js

## Supported Languages/Charsets
- Latin
- Chinese, Korean, Japanese (CJK)
- Hindi
- Arabic
- Cyrillic
- Greek and Coptic
- Hebrew

## API Overview

### Main Classes
- `Index` - Basic search index
- `Document` - Multi-field document search
- `Worker` - Background processing
- `Encoder` - Text encoding/transformation
- `Resolver` - Complex query composition
- `IndexedDB` - Persistent storage interface

### Key Methods
- `add(id, content)` - Add content to index
- `search(query, options)` - Perform search
- `update(id, content)` - Update indexed content
- `remove(id)` - Remove from index
- `clear()` - Clear entire index

## File Structure
The project follows a modular approach with:
- Source code in `src/` directory
- Different builds in `dist/` directory
- Comprehensive documentation in `doc/` directory
- Examples for different use cases in `example/` directory
- Build tasks in `task/` directory

This structure enables flexible usage across different environments and requirements while maintaining high performance and extensive feature support.