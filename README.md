# Media Converter

A powerful and intuitive desktop media converter application built with modern technologies. Convert, resize, and compress your images and videos with ease.

## 🎯 What It Does

**Media Converter** is a cross-platform desktop application that helps you:

- **Convert** images and videos between popular formats
- **Resize** media files to custom dimensions
- **Compress** files to reduce size while maintaining quality
- **Batch process** multiple files at once

## ✨ Key Features

### 🖼️ Image Conversion

- **Supported Formats**: JPEG, PNG, WebP, BMP, TIFF, ICO
- **Smart Resizing**: Maintain aspect ratio or set custom dimensions
- **Batch Processing**: Convert multiple images simultaneously

### 🎬 Video Conversion

- **Supported Formats**: MP4, AVI, MOV, WebM, MKV
- **Resolution Control**: Resize videos to any dimension
- **Duration Management**: Trim or compress video length
- **Frame Rate Control**: Adjust FPS for smooth playback
- **Audio Options**: Keep or remove audio tracks

### 🚀 User Experience

- **Real-time Preview**: See conversion progress and status
- **Custom Output Names**: Rename files during conversion
- **Flexible Output**: Choose destination folders or use defaults
- **Visual Feedback**: Color-coded status indicators (converting, completed, error)

## 🎨 Interface

### Clean and Intuitive Design

- **File Upload Area**: Drag and drop or click to select files
- **File List**: View all selected files with thumbnails and details
- **Options Panel**: Customize conversion settings for each file

## 🔧 How It Works

1. **Select Files**: Drag and drop or browse for your media files
2. **Choose Settings**: Select output format, dimensions, and quality
3. **Customize Output**: Set custom filenames and destination folders
4. **Convert**: Click convert and watch the progress in real-time
5. **Done**: Files are saved to your chosen location with visual confirmation

## � Use Cases

### For Content Creators

- Resize images for social media platforms
- Convert videos to web-friendly formats
- Compress large files for faster uploads
- Batch process photo collections

### For Developers

- Optimize images for web applications
- Convert media assets for different platforms
- Reduce file sizes for mobile applications
- Prepare media for content delivery networks

### For General Users

- Reduce photo storage space
- Convert videos for device compatibility
- Prepare media for email attachments
- Archive photos in efficient formats

## 🏆 Why Choose Media Converter?

- **Fast & Efficient**: Native desktop performance with Rust backend
- **User-Friendly**: Intuitive interface designed for all skill levels
- **Reliable**: Built-in error handling and status reporting
- **Flexible**: Comprehensive format support and customization options
- **Safe**: Local processing - your files never leave your computer
- **Free**: Open source and completely free to use

## 🛠️ Technical Foundation

Built with modern, reliable technologies:

- **Frontend**: Nuxt 3 + Vue.js for responsive UI
- **Backend**: Tauri + Rust for fast, secure processing
- **Media Processing**: FFmpeg integration for professional-grade conversion
- **UI Framework**: Nuxt UI for beautiful, accessible components

## � Getting Started

### Installation

1. Download the latest release for your operating system
2. Install the application following standard procedures
3. Launch and start converting your media files immediately

### Quick Start

1. **Add Files**: Drag your images or videos into the app
2. **Select Format**: Choose your desired output format
3. **Adjust Settings**: Set dimensions, quality, and output location
4. **Convert**: Click the convert button and wait for completion

## � System Requirements

- **Windows**: Windows 10 or later
- **macOS**: macOS 10.15 or later
- **Linux**: Most modern distributions
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 100MB for installation + space for converted files

## �️ For Developers

If you want to contribute or build from source:

### Prerequisites

- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- [Yarn](https://yarnpkg.com/) package manager

### Development Setup

```bash
# Install dependencies
yarn install

# Run in development mode
yarn tauri:dev

# Build for production
yarn tauri:build
```

## 📁 Project Structure

```
├── components/           # Vue components
├── pages/               # Application pages
├── stores/              # Pinia state management
├── utils/               # Utility functions
├── src-tauri/           # Rust backend
└── types/               # TypeScript definitions
```

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Report Issues**: Found a bug? Let us know!
2. **Feature Requests**: Have an idea? We'd love to hear it!
3. **Code Contributions**:
   - Fork the repository
   - Create a feature branch
   - Make your changes
   - Test thoroughly
   - Submit a pull request

### Development Guidelines

- Test with `yarn tauri:dev` before submitting
- Ensure builds work with `yarn tauri:build`
- Follow existing code style and patterns
- Add appropriate documentation

## 📞 Support

- **Issues**: Report bugs or request features on GitHub
- **Documentation**: Check the wiki for detailed guides
- **Community**: Join discussions in GitHub Discussions

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

---

**Download Media Converter today and start converting your media files with ease!**
