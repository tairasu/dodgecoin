using Godot;

public partial class StartButton : Button
{

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{
		//START LEVEL
		if(ButtonPressed) {
			//print to console
			GD.Print("Start button pressed");
			GetTree().ChangeSceneToFile("res://scenes/levels/level1.tscn");
		}
	}
}
