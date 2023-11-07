
var maps = [
    [
        {x: 50, y: 0, rot: -Math.PI/2},
        [
            [0, 100],
            [0, -100],
            [100, -200],
            [1100, -200],
            [1200, -100],
            [1200, 100],
            [1100, 200],
            [100, 200],
            [0, 100],
        ],
        [0, 4]
    ],
    [
        {x: 50, y: 0, rot: 0},
        [
            [0, 100],
            [0, -100],
            [100, -200],
            [1100, -200],
            [1300, 0],
            [1300, 1000],
            [1200, 1100],
            [1000, 1100],
            [900, 1000],
            [900, 300],
            [800, 200],
            [100, 200],
            [0, 100],
        ],
        [0, 6]
    ],
    [
        {x: 50, y: 0, rot: 0},
        [
            [0, 100],
            [0, -100],
            [100, -200],
            [1100, -200],
            [1300, 0],
            [1300, 1100],
            [1400, 1200],
            [2500, 1200],
            [2600, 1300],
            [2600, 1500],
            [2500, 1600],
            [1100, 1600],
            [900, 1400],
            [900, 300],
            [800, 200],
            [100, 200],
            [0, 100],
        ],
        [0, 8]
    ],
    [
        {x: 50, y: 0, rot: 0},
        [
            [0, 100],
            [0, -100],
            [100, -200],
            [1100, -200],
           
        ],
        [0, 2]
    ]
]

var map = []
var mapData = []
var mapIndex = 0

function loadMap(index) {
    map = maps[index].slice(1, maps[index].length-1)
    mapData = maps[index]
    player.x = maps[index][0].x
    player.y = maps[index][0].y
    player.velX = 0
    player.velY = 0
    player.rot = maps[index][0].rot
    finished = false
    timing = false
    time = 0
    mapIndex = index
}