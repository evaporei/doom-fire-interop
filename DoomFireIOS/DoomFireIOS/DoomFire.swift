//
//  Doom.swift
//  DoomFireIOS
//
//  Created by Otavio Paulino Pace on 3/20/19.
//  Copyright © 2019 Otavio Pace. All rights reserved.
//

import Foundation

struct Callbacks {
    static var render: (([UInt8]) -> Void)!
}

class DoomFire {
    var pixelBoard: COpaquePointer
    
    init(width: Int, height: Int) {
        pixelBoard = create_board(width, height)
    }
    
    func createFireSource() {
        create_fire_source(pixelBoard)
    }
    
    func calculateFirePropagation(render: ([UInt8]) -> Void) {
        if Callbacks.render == nil {
            Callbacks.render = render
        }
        
        calculate_fire_propagation(pixelBoard, { (pixelsPtr, size) in
            var pixelsArray = Array(UnsafeBufferPointer(start: pixelsPtr, count: size))
            pixelsArray = pixelsArray.reverse()
            Callbacks.render(pixelsArray)
        })
    }
    
    func getPixelsArray() -> [UInt8] {
        let pixelsPtr = get_pixels_ptr(pixelBoard)
        let size = get_pixels_len(pixelBoard)
        
        var pixelsArray = Array(UnsafeBufferPointer(start: pixelsPtr, count: size))
        
        pixelsArray = pixelsArray.reverse()
        
        return pixelsArray
    }
    
    deinit {
        free_board(pixelBoard)
    }
}