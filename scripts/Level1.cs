using System.Linq;
using Godot;

public partial class Level1 : Node
{
	[Export]
	PackedScene[] rooms;
	
	Node2D startingRoom;

	CharacterBody2D player;

	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		var roomScene = rooms[GD.Randi() % rooms.Length];
		startingRoom = (Node2D)roomScene.Instantiate();

		float rotation = GD.Randi() % 4 * (Mathf.Pi / 2);
		startingRoom.Rotation = rotation;

		AddChild(startingRoom);

		//GenerateSurroundingRooms(startingRoom);
	}
	

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{

	}

	private void GenerateSurroundingRooms(Node2D currentRoom)
	{
		var roomScene = rooms[GD.Randi() % rooms.Length];
		Node2D room = (Node2D)roomScene.Instantiate();

		float rotation = GD.Randi() % 4 * (Mathf.Pi / 2);
		room.Rotation = rotation;

		
		room.Position = currentRoom.Position + 550 * Vector2.Left;
		

	}

	
}
