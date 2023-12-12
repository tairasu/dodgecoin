using Godot;

public partial class Level1 : Node
{
	[Export]
	PackedScene[] rooms;
	
	Vector2 currentRoomPos;

	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		// var roomScene = rooms[GD.Randi() % rooms.Length];
		// Node2D startingRoom = (Node2D)roomScene.Instantiate();
		// Room roomArea = startingRoom.GetNode<Room>("Area2D");
		// roomArea.PlayerEntered += OnPlayerEntered;
		// AddChild(startingRoom);
		GenerateSurroundingRooms(Vector2.Zero);

		//CharacterBody2D player = GetNode<CharacterBody2D>("%Player");
	}

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{

	}

	private void GenerateSurroundingRooms(Vector2 pos)
	{
		var roomScene = rooms[GD.Randi() % rooms.Length];

		for (int i = -1; i <= 1; i++) {
			for (int j = -1; j <= 1; j++) {
				Node2D room = (Node2D)roomScene.Instantiate();
				room.Position = pos + 550 * i * Vector2.Right + 550 * j * Vector2.Down;
				Room roomArea = room.GetNode<Room>("Area2D");
				if (i != 0 || j != 0)	roomArea.PlayerEntered += OnPlayerEntered;
				AddChild(room);
			}
		}
	}

	private void OnPlayerEntered(Vector2 pos)
	{
		currentRoomPos = pos;
		GD.Print("Player entered room at position " + currentRoomPos.ToString());
		foreach (Node2D node in GetChildren()) {
			if (node.IsInGroup("Level")) {
				node.CallDeferred(MethodName.Free);
			}
		}
		CallDeferred(MethodName.GenerateSurroundingRooms, currentRoomPos);
	}

}
