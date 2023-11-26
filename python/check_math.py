import matplotlib.pyplot as plt
import numpy as np

# Of the form [x1, x2, x3, x4], and [y1, y2, y3, y4]
def do_segments_intersect(x : list[float], y : list[float]) -> bool:
    [x1, x2, x3, x4] = x
    [y1, y2, y3, y4] = y

    if min(x1,x2) > max(x3,x4): return False

    a1 = (y1 - y2) / (x1 - x2)
    a2 = (y3 - y4) / (x3 - x4)
    b1 = y1 - a1*x1
    b2 = y3 - a2*x3

    if a1 == a2:
        return False

    Xa = (b2 - b1) / (a1 - a2)

    if ( (Xa < max( min(x1,x2), min(x3,x4) )) or (Xa > min( max(x1,x2), max(x3,x4) )) ):
        return False  # intersection is out of bound
    else:
        return True


# Define your line segments
x_segments = [[1, 4], [-2, -2]]
y_segments = [[2, 6], [7, 4]]

# Plot each line segment
for x_seg, y_seg in zip(x_segments, y_segments):
    plt.plot(x_seg, y_seg, marker='o')

# Figure out normal vectors and plot them for one line segment
dx = x_segments[0][1] - x_segments[0][0]
dy = y_segments[0][1] - y_segments[0][0]

norm_1 = [-dy, dx]
norm_2 = [dy, -dx]

plt.plot(norm_1[0], norm_1[1], marker='o', color='red')
plt.plot(norm_2[0], norm_2[1], marker='o', color='green')

second_line_pt1 = [x_segments[1][0], y_segments[1][0]]
second_line_pt2 = [x_segments[1][1], y_segments[1][1]]

if do_segments_intersect([x_segments[0][0], x_segments[0][1], x_segments[1][0], x_segments[1][1]],
                         [y_segments[0][0], y_segments[0][1], y_segments[1][0], y_segments[1][1]]):
    print("LINES INTERSECT")
else:
    print("LINES DO NOT INTERSECT")

# Calculate dot product between the normal vector and the other lines
print(np.dot(norm_1, second_line_pt1))
print(np.dot(norm_1, second_line_pt2))
print(np.dot(norm_2, second_line_pt1))
print(np.dot(norm_2, second_line_pt2))

if (np.sum([np.dot(norm_1, second_line_pt1), np.dot(norm_1, second_line_pt2)]) >
    np.sum([np.dot(norm_2, second_line_pt1), np.dot(norm_2, second_line_pt2)])):
    print('Norm1 [red] is the right direction')
else:
    print('Norm2 [green] is the right direction.')

# Set axis labels and title
plt.xlabel('X-axis')
plt.ylabel('Y-axis')
plt.title('2D Line Segments')

# Show the plot
plt.show()
