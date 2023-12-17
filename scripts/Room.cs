using System.Linq;
using Godot;
using Godot.Collections;

public partial class Room : Node2D
{
	public Vector2 id { get; set; }
	public float width = 600;
	public float height = 600;
	private Dictionary openDoors = new Dictionary
	{
		{"left", true},
		{"right", true},
		{"up", true},
		{"down", true}
	};
	private int openDoorCount;

	public override void _Ready()
	{
		updateOpenDoorCount();
	}

    public void updateOpenDoorCount()
	{
		openDoorCount = openDoors.Values.Sum(x => (bool)x ? 1 : 0);
	}



}
