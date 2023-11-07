using Godot;

public partial class quit_button : Button
{
	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
	}

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{
		//QUIT GAME
		if(ButtonPressed) {
			GetTree().Quit();
		}
	}
}
