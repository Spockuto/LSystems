var lObject1 = new Object();
lObject1["variables"] = "XF";
lObject1["constants"] = "+−[]";
lObject1["angle"] = 30;
lObject1["round"] = 7;
lObject1["axiom"] = "X";
//Shoud be of the form X = XY;
lObject1["rule1"] = "X=F+[[X]-X]-F[-FX]+X";
lObject1["rule2"] = "F=FF";
lObject1["rule3"] = "";

var lObject2 = new Object();
lObject2["variables"] = "XY";
lObject2["constants"] = "+−[]";
lObject2["angle"] = 90;
lObject2["round"] = 14;
lObject2["axiom"] = "FX";
//Shoud be of the form X = XY;
lObject2["rule1"] = "X=X+YF+";
lObject2["rule2"] = "Y=-FX-Y";
lObject2["rule3"] = "";


var lObject3 = new Object();
lObject3["variables"] = "XY";
lObject3["constants"] = "+−[]";
lObject3["angle"] = 90;
lObject3["round"] = 4;
lObject3["axiom"] = "XYXYXYX+XYXYXYX+XYXYXYX+XYXYXYX";
//Shoud be of the form X = XY;
lObject3["rule1"] = "F=";
lObject3["rule2"] = "X=FX+FX+FXFY-FY-";
lObject3["rule3"] = "Y=+FX+FXFY-FY-FY";

var lObject4 = new Object();
lObject4["variables"] = "F";
lObject4["constants"] = "+−[]";
lObject4["angle"] = 22.5;
lObject4["round"] = 4;
lObject4["axiom"] = "F";
//Shoud be of the form X = XY;
lObject4["rule1"] = "F=FF-[-F+F+F]+[+F-F-F]";
lObject4["rule2"] = "";
lObject4["rule3"] = "";

var lObject = lObject1;
var ruleDict = new Object();
var stack = [];
var stackObject = new Object();
stackObject["x"] = "";
stackObject["y"] = "";
stackObject["angle"] = "";
var canvasHeight;
function selectFractal(i){
    if(i == 1){
        lObject = lObject1;
        canvasHeight = 0;
    }

    if(i == 2){
        lObject = lObject2;
        canvasHeight = 200;
    }

    if(i == 3){
        lObject  = lObject3;  
        canvasHeight = window.innerHeight/2;  
    }

    if(i == 4){
        lObject = lObject4;
        canvasHeight = 0;
    }
}

function generateRuleDict(){
    ruleDict[lObject.rule1.split("=")[0]] = lObject.rule1.split("=")[1];
    ruleDict[lObject.rule2.split("=")[0]] = lObject.rule2.split("=")[1];
    ruleDict[lObject.rule3.split("=")[0]] = lObject.rule3.split("=")[1];
}

function generateSequence(){
    lObject.round = parseInt(document.getElementById('iterator').value);
    if(lObject.round == null)
        lObject.round = 5;
    var round = lObject.round;
    var sequence;
    var variables = lObject.variables.split("");
    generateRuleDict();

    for(var i  = 0 ; i < round; i++ ){
        if(i == 0){
            sequence = lObject["axiom"];
        }
        else{
            for(var j = 0; j < sequence.length;){
                if(variables.indexOf(sequence[j]) != -1){
                    var length = sequence.length;
                    var char = sequence[j];
                    sequence = sequence.substr(0,j) + ruleDict[sequence[j]] + sequence.substr(j+1, length);
                    j = j + ruleDict[char].length;
                }
                else{
                    j++;
                }
            }
        }
    }
    return sequence;
}

function drawStuff() {
        context.moveTo(0,0);
        var r = 30/lObject.round;
        var lastPosX = 0;
        var lastPosY = 0;
        var lastangle = lObject.angle;
        var angleplus = Math.PI * lObject["angle"] / 180.0;
        var angleminus = -1 * Math.PI * lObject["angle"] / 180.0;
        var sequence = generateSequence();
        for(var i = 0 ; i < sequence.length; i++){
            if(sequence[i] == 'F'){
                lastPosX = lastPosX + r * Math.cos(lastangle);
                lastPosY = lastPosY + r * Math.sin(lastangle);
                context.lineTo(lastPosX, lastPosY);
                context.stroke();        
            }
            else if(sequence[i] == '-') {
                lastangle = lastangle + angleplus;
                
            }
            else if(sequence[i] == '+'){
                lastangle = lastangle + angleminus;
            }
            else if(sequence[i] == '['){
                var stackObject = new Object();
                stackObject["x"] = "";
                stackObject["y"] = "";
                stackObject["angle"] = "";
                stackObject.x = lastPosX;
                stackObject.y = lastPosY;
                stackObject.angle = lastangle;
                stack.push(stackObject);
            }
            else if(sequence[i] ==']'){
                var obj = stack.pop();
                lastPosX = obj.x;
                lastPosY = obj.y;
                lastangle = obj.angle;
                context.moveTo(lastPosX, lastPosY);
            }   
        }
    }

    var canvas = document.getElementById('lCanvas'),
    context = canvas.getContext('2d');

    
    window.addEventListener('resize', resizeCanvas, false);
    function resizeCanvas() {
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight - 100;
            context.translate(canvas.width/2,canvas.height - canvasHeight);
            drawStuff(); 
    }
    resizeCanvas();
    