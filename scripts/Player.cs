using Godot;

public partial class Player : CharacterBody2D
{
	private float currentSpeed = 0.0f;
	private const float maxSpeed = 600.0f;
	private const float accelerationTime = 0.2f; // time until max speed is reached
	private float accelerationTimer = 0.0f;

	public override void _PhysicsProcess(double delta)
	{
		Vector2 velocity = Velocity;

		Vector2 direction = Input.GetVector("ui_left", "ui_right", "ui_up", "ui_down");
		if (direction != Vector2.Zero)
		{
			// Increase the acceleration timer
			accelerationTimer += (float)delta;
			// Calculate the current speed using Lerp
			currentSpeed = Mathf.Lerp(0, maxSpeed, accelerationTimer / accelerationTime);
			// Apply the current speed to the velocity
			velocity.X = direction.X * currentSpeed;
			velocity.Y = direction.Y * currentSpeed;
		}
		else
		{
			// Reset the acceleration timer and current speed when no key is pressed
			accelerationTimer = 0.0f;
			currentSpeed = 0.0f;
			velocity.X = Mathf.MoveToward(Velocity.X, 0, 0);
			velocity.Y = Mathf.MoveToward(Velocity.Y, 0, 0);
		}

		Velocity = velocity;
		MoveAndSlide();
	}
}
