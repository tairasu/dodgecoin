using Godot;
using System;
using System.Runtime.CompilerServices;

public partial class Coin : StaticBody2D
{
	private const float Speed = 400.0f;
	//make velocity vector
	private Vector2 velocity = new Vector2(0,0);

	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		//make object move in a random direction
		Random rand = new Random();
		velocity = new Vector2(rand.Next(-1, 1), rand.Next(-1, 1));
		velocity = velocity.Normalized();
		//set velocity
		

	}

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{
		//rotate coin
		RotationDegrees += 5;

		//check collision
		KinematicCollision2D collision = MoveAndCollide(velocity * Speed * (float)delta);
		if(collision != null) {
			velocity = velocity.Bounce(collision.GetNormal());
		}
	}

}