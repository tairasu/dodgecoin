using Godot;

public partial class Level1 : Node
{
	[Export]
	PackedScene roomScene;

	Vector2[,] roomIDs = new Vector2[5, 5];
	Vector2 currentRoomID;



	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		GenerateSurroundingRooms(Vector2.Zero);
	}

	
	/// <summary>
	/// Generates surrounding rooms based on the given position.
	/// </summary>
	/// <param name="pos">The position to generate the rooms around.</param>
	private void GenerateSurroundingRooms(Vector2 pos)
	{
		for (int i = -2; i <= 2; i++)
		{
			for (int j = -2; j <= 2; j++)
			{
				Room room = (Room)roomScene.Instantiate();
				room.Position = pos + room.width * i * Vector2.Right + room.height * j * Vector2.Down;
				room.Set(
					"id",
					new Vector2(
						room.Position.X / room.width,
						room.Position.Y / room.height
					)
				);

				RoomArea roomArea = room.GetNode<RoomArea>("Area2D");
				if (i != 0 || j != 0) roomArea.PlayerEntered += OnPlayerEntered;
				AddChild(room);
			}
		}
	}

	private void OnPlayerEntered(Vector2 pos, Vector2 id)
	{
		currentRoomID = id;
		 GD.Print("Player entered room with ID " + currentRoomID);
		foreach (Node2D node in GetChildren())
		{
			if (node.IsInGroup("Level"))
			{
				node.CallDeferred(MethodName.Free);
			}
		}
		CallDeferred(MethodName.GenerateSurroundingRooms, pos);
	}

}
