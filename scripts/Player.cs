using Godot;

public partial class Player : CharacterBody2D
{
	[Export]
	private int speed = 500;
	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{

	}

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{
		Vector2 direction = Input.GetVector("ui_left", "ui_right", "ui_up", "ui_down");
		Vector2 motion = direction * speed * (float)delta;
		MoveAndCollide(motion);
	}
}
