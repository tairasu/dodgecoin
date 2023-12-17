using Godot;

public partial class QuitButton : Button
{

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{
		//QUIT GAME
		if(ButtonPressed) {
			GetTree().Quit();
		}
	}
}
