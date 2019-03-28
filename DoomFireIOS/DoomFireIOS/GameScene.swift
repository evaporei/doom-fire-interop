//
//  GameScene.swift
//  DoomFireIOS
//
//  Created by Otavio Paulino Pace on 3/18/19.
//  Copyright (c) 2019 Otavio Pace. All rights reserved.
//

import SpriteKit

struct Singleton {
    static var nodes: [SKShapeNode]!
}

class GameScene: SKScene {
    var doomFire: DoomFire!
    
    let nodeSize = 7
    
    override func didMoveToView(view: SKView) {
        super.didMoveToView(view)
        
        let width = Int(view.frame.width) / nodeSize + (nodeSize / 2)
        let height = Int(view.frame.height) / nodeSize + (nodeSize / 2)
        
        doomFire = DoomFire(width: width, height: height)
        
        doomFire.createFireSource()
        
        let pixelsArray = doomFire.getPixelsArray()
        
        Singleton.nodes = pixelsArray.enumerate().map { (i, pixel) in
            let node = self.makeNode(Int(pixel), x: CGFloat(i % width), y: CGFloat(i / width))
            self.addChild(node)
            
            return node
        }
        
        let propagateFire = SKAction.runBlock {
            self.doomFire.calculateFirePropagation({ (pixels) in
                Singleton.nodes.enumerate().forEach { index, node in
                    node.fillColor = DoomPixelColor.pixelColor(
                        forIntensity: Int(pixels[index]))
                }
            })
        }
        
        let propagateFireWithDelay = SKAction.sequence([
            propagateFire,
            SKAction.waitForDuration(0.1)
            ])
        
        runAction(SKAction.repeatActionForever(propagateFireWithDelay))
    }
    
    private func makeNode(intensity: Int, x: CGFloat, y: CGFloat) -> SKShapeNode {
        let node = SKShapeNode(rectOfSize: CGSize(width: nodeSize, height: nodeSize))
        node.strokeColor = .clearColor()// .clear
        node.fillColor = DoomPixelColor.pixelColor(forIntensity: intensity)
        node.position = CGPoint(
            x: Int(x) * nodeSize + (nodeSize / 2),
            y: Int(y) * nodeSize + (nodeSize / 2))
        return node
    }
}
