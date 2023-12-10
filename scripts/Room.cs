using Godot;

public partial class Room : Node2D
{
	private bool leftOpen;
	private bool rightOpen;
	private bool topOpen;
	private bool bottomOpen;

	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		RayCast2D rayLeft = GetNode<RayCast2D>("ray_left");
		RayCast2D rayRight = GetNode<RayCast2D>("ray_right");
		RayCast2D rayUp = GetNode<RayCast2D>("ray_up");
		RayCast2D rayDown = GetNode<RayCast2D>("ray_down");
		var spaceState = GetWorld2D().DirectSpaceState;
		var query = PhysicsRayQueryParameters2D.Create(Vector2.Zero, new Vector2(-350, 0));
    	var result = spaceState.IntersectRay(query);

		GD.Print(result);
	}

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{
	}
}
