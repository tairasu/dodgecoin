using Godot;

public partial class Room : Area2D
{
	[Signal]
	public delegate void PlayerEnteredEventHandler(Vector2 pos);

	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		BodyEntered += OnBodyEntered;
	}

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{
	}

	private void OnBodyEntered(Node2D body)
	{
		EmitSignal(nameof(PlayerEntered), GlobalPosition);
	}


}
