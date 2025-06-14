import AppKit
import Foundation

// Function to set wallpaper for all screens
func setWallpaperForAllScreens(imagePath: String) {
    let imageUrl = URL(fileURLWithPath: imagePath)

    // Check if the file exists
    guard FileManager.default.fileExists(atPath: imagePath) else {
        print("Error: Image file not found at \(imagePath)")
        exit(1)
    }

    let workspace = NSWorkspace.shared
    let screens = NSScreen.screens

    if screens.isEmpty {
        print("Error: No screens found.")
        exit(1)
    }

    var successCount = 0
    var errorCount = 0

    for screen in screens {
        do {
            // For different scaling options, you can modify the options dictionary.
            // For example, to scale to fit:
            // let options: [NSWorkspace.DesktopImageOptionKey: Any] = [
            //    .imageScaling: NSImageScaling.scaleProportionallyUpOrDown.rawValue,
            //    .allowClipping: true
            // ]
            // For this basic version, we'll use default options.
            try workspace.setDesktopImageURL(imageUrl, for: screen, options: [:])
            print("Successfully set wallpaper for screen: \(screen.localizedName)")
            successCount += 1
        } catch {
            print("Error setting wallpaper for screen \(screen.localizedName): \(error.localizedDescription)")
            errorCount += 1
        }
    }

    if errorCount > 0 {
        print("Finished setting wallpapers with \(errorCount) error(s) and \(successCount) success(es).")
        if successCount == 0 {
             exit(1) // Exit with error if no wallpaper was set successfully
        }
    } else {
        print("Successfully set wallpaper for all \(successCount) screen(s).")
    }
}

// --- Main execution ---
if CommandLine.arguments.count < 2 {
    print("Usage: wallpaper <image_path>")
    print("Example: wallpaper /path/to/your/image.jpg")
    exit(1)
}

let imagePathArgument = CommandLine.arguments[1]

setWallpaperForAllScreens(imagePath: imagePathArgument)
