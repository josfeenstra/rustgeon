// author: Jos Feenstra
// inspiration from Three.js
// note: recycle the class as much as possble, 


struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {

    pub fn zero() -> Self {

    }

    pub fn new(x: f32, y:f32, z:f32) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn from_lerp(v1: Self, v2: Self, v3: Self) -> Self {
        return new(
			v1.x + ( v2.x - v1.x ) * alpha,
            v1.y + ( v2.y - v1.y ) * alpha,
            v1.z + ( v2.z - v1.z ) * alpha
		);
    }

    pub fn from_array() -> Self {
           
    }

    pub fn from_random() -> Self {

    }



}

export class Vector3
{
	static fromLerp( v1:Vector3, v2:Vector3, alpha:number ) : Vector3 
	{
		return this.constructor(
			v1.x + ( v2.x - v1.x ) * alpha,
            v1.y + ( v2.y - v1.y ) * alpha,
            v1.z + ( v2.z - v1.z ) * alpha
		);
	}

    static fromArray(a: Array<number>) : Vector3 {
        return new Vector3(a[0], a[1], a[2]);
    }

    static fromRandom() : Vector3 
    {
        return this.constructor(
		    Math.random(),
		    Math.random(),
		    Math.random()
        )
	}

    static fromSphere(radius:number, theta:number, phi:number) : Vector3
    {
		const sinPhiRadius = Math.sin( phi ) * radius;
        return this.constructor(
            sinPhiRadius * Math.sin( theta ),
            Math.cos( phi ) * radius,
            sinPhiRadius * Math.cos( theta ),
        );
	}

    static fromCylinder(radius:number, theta:number, height:number) : Vector3 
    {
        return this.constructor(
            radius * Math.sin( theta ),
            height,
            radius * Math.cos( theta )
        );
	}

    // #endregion
	// #region basics

	set(x:number, y:number, z:number) : Vector3
	{
		this.x = x;
        this.y = y;
        this.z = z;
		return this 		
	}

	equals(v:Vector3) : boolean
	{
		return ( ( v.x === this.x ) && ( v.y === this.y ) && ( v.z === this.z ) );
	}

    toString() : string
    {
        return `Vector3(${this.x}, ${this.y}, ${this.z})`;
    }

    clone() : Vector3
    {
        return this.constructor(this.x, this.y, this.z);
    }

    copy(v: Vector3) : Vector3
    {
        this.x = v.x;
        this.y = v.y;
        this.z = v.z;
        return this;
    }

	to2D(): Vector2 {
        return new Vector2(this.x, this.y);
    }

    // #endregion
	// #region math like vector2

    add(v: Vector3) : Vector3
    {
        this.x += v.x;
        this.y += v.y;
        this.z += v.z;
        return this;
    }

    sub(v: Vector3) : Vector3
    {
        this.x -= v.x;
        this.y -= v.y;
        this.z -= v.z;
        return this;
    }


    mul(v: Vector3) : Vector3
    {
        this.x += v.x;
        this.y += v.y;
        this.z == v.z;
        return this;
    }

    scale(v: number) : Vector3
    {
        this.x *= v;
        this.y *= v;
        this.z *= v;
        return this;
    }

    div(v: Vector3) : Vector3
    {
        this.x /= v.x;
        this.y /= v.y;
        this.z /= v.z;
        return this;
    }

    divscale(v: number) : Vector3
    {
        this.x /= v;
        this.y /= v;
        this.z /= v;
        return this;
    }

    min(other: Vector3) : Vector3 
    {
		this.x = Math.min( this.x, other.x );
        this.y = Math.min( this.y, other.y );
        this.z = Math.min( this.z, other.z );
		return this;
	}

	max(other: Vector3) : Vector3 
	{
		this.x = Math.max( this.x, other.x );
        this.y = Math.max( this.y, other.y );
        this.z = Math.max( this.z, other.z );
		return this;
	}

	clamp(min: Vector3, max: Vector3) : Vector3 
	{
		// assumes min < max, componentwise
		this.x = Math.max(min.x, Math.min(max.x, this.x));
        this.y = Math.max(min.y, Math.min(max.y, this.y));
        this.z = Math.max(min.z, Math.min(max.z, this.z));
		return this;
	}

	clampScalar(minVal:number, maxVal:number) : Vector3 
	{
		this.x = Math.max( minVal, Math.min(maxVal, this.x));
        this.y = Math.max( minVal, Math.min(maxVal, this.y));
        this.z = Math.max( minVal, Math.min(maxVal, this.z));
		return this;
	}

	clampLength(min:number, max:number) : Vector3
	{
		const length = this.length();
		return this.divscale( length || 1 ).scale( Math.max(min, Math.min(max, length)));
	}

	floor() : Vector3 
	{
		this.x = Math.floor(this.x);
        this.y = Math.floor(this.y);
        this.z = Math.floor(this.z);
		return this;
	}

	ceil() : Vector3 
	{
		this.x = Math.ceil(this.x);
        this.y = Math.ceil(this.y);
        this.z = Math.ceil(this.z);
		return this;
	}

	round() : Vector3 
	{
		this.x = Math.round(this.x);
        this.y = Math.round(this.y);
        this.z = Math.round(this.z);
		return this;
	}

	roundToZero() : Vector3 
	{
		this.x = (this.x < 0) ? Math.ceil( this.x ) : Math.floor( this.x );
        this.y = (this.y < 0) ? Math.ceil( this.y ) : Math.floor( this.y );
        this.z = (this.z < 0) ? Math.ceil( this.z ) : Math.floor( this.z );
		return this;
	}

	negate() : Vector3 
	{
		this.x = -this.x;
        this.y = -this.y;
        this.z = -this.z;
		return this;
	}

	dot( v:Vector3 ) : number 
	{
		return this.x * v.x + this.y * v.y + this.z * v.z;
	}

	cross( other:Vector3 ) : Vector3
	{
        const ax = this.x, ay = this.y, az = this.z;
		const bx = other.x, by = other.y, bz = other.z;

		this.x = ay * bz - az * by;
		this.y = az * bx - ax * bz;
		this.z = ax * by - ay * bx;

		return this;
	}

	getLengthSquared() : number
	{
		return this.x * this.x + this.y * this.y + this.z * this.z;
	}

	length() : number
	{
		return Math.sqrt(this.getLengthSquared());
	}

	manhat() : number
	{
		return Math.abs( this.x ) + Math.abs( this.y ) + Math.abs( this.z );
	}

	normalize() : Vector3
	{
		return this.divscale( this.length() || 1 );
	}

	disTo(v: Vector3) : number
	{
		return Math.sqrt(this.disToSquared(v));
	}

	disToSquared(v: Vector3) : number
	{
		const dx = this.x - v.x, 
              dy = this.y - v.y,
              dz = this.z - v.z;
		return dx * dx + dy * dy + dz * dz;
	}

	disToManhat(v: Vector3) : number
	{
        return Math.abs(this.x - v.x) 
             + Math.abs(this.y - v.y) 
             + Math.abs(this.z - v.z);
	}

	setLength(length: number) : Vector3
	{
		return this.normalize().scale(length);
	}

	lerp(other: Vector3, alpha:number) : Vector3
	{
		this.x += ( other.x - this.x ) * alpha;
        this.y += ( other.y - this.y ) * alpha;
        this.z += ( other.z - this.z ) * alpha;
		return this;
    }
    
    // #endregion
    // #region math specific 

    projectOnVector(other: Vector3) 
    {
        // use dot product to project this vector on the other vector 
		const denominator = other.getLengthSquared();
		if ( denominator === 0 ) return this.set( 0, 0, 0 );
		const scalar = other.dot(this) / denominator;
		return this.copy(other).scale(scalar);
	}

    projectOnPlane(normal: Vector3) 
    {
        // project a vector 
		_vector.copy(this).projectOnVector(normal);
		return this.sub(_vector);
	}

    mirror(normal: Vector3) 
    {
		// mirror incident vector off plane orthogonal to normal
		// normal is assumed to have unit length
		return this.sub(_vector.copy(normal).scale(2 * this.dot(normal)));
	}

    // #endregion
    // TODO : matrix interactions
}

// this is an optimization trick used by three.js, dont know why 
const _vector = new Vector3(0,0,0);