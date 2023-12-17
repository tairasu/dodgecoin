using Godot;

public partial class RoomArea : Area2D
{
	[Signal]
	public delegate void PlayerEnteredEventHandler(Vector2 pos, Vector2 id);

	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		BodyEntered += OnBodyEntered;
	}

	private void OnBodyEntered(Node2D body)
	{
		EmitSignal(nameof(PlayerEntered), GlobalPosition, GetParent<Room>().id);
	}


}
