float sd_plane(vec3 pos, vec3 n, float h)
{
	return dot(pos, n) + h;
}

float sd_sphere(vec3 pos, vec3 center, float rad)
{
	return length(pos - center) - rad;
}

float sd_round_box(vec3 p, vec3 b)
{
  vec3 q = abs(p) - b;
  return length(max(q, 0.0)) + min(max(q.x ,max(q.y,q.z)), 0.0);
}

float sd_box(vec3 pos, vec3 size)
{
	vec3 d = abs(pos) - size;
	return max(d.x, max(d.y, d.z));
}