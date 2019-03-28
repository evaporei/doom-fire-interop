//
//  GameViewController.swift
//  DoomFireIOS
//
//  Created by Otavio Paulino Pace on 3/18/19.
//  Copyright (c) 2019 Otavio Pace. All rights reserved.
//

import UIKit
import SpriteKit

class GameViewController: UIViewController {
    override func viewDidLoad() {
        super.viewDidLoad()
        
        let skView = SKView(frame: view.bounds)
        skView.showsFPS = true
        skView.showsNodeCount = true
        
        view.addSubview(skView)
        
        let scene = GameScene(size: skView.frame.size)
        skView.presentScene(scene)
    }
}
