#import "Latex-Macros/lib.typ": *
#show: setup.with(
  title: "Quaternion Algrebra",
  bib: true,
  header-center: "Quaternions",
  numbering: (equation: true, section: true),
)

#let ab(body) = {
  math.arrow(math.bold(body))
}

#metadata("title") <titlepage>
#align(center)[#title()\ #author \ #datetime.today().display("[month repr:long] [day], [year]")]\
#line(length: 100%)\
#outline(depth: 2)
#pagebreak()

= Overview
A quaternion is given to be
$ q = w + x i + y j + z k $<def>
$ w, x, y, z in RR $
The set of all quaternions is denoted $H$ and the set of all unit quaternions is denoted $H_1$.

The imaginary elements $i, j, k$ are additionally defined as intuitive extensions to the familiar
imaginary number@eberly2002:
$
  i^2 & = j^2  && = k^2 = -1 \
  i j & = -j i && = k \
  j k & = -k j && = i \
  k i & = -i k && = j
$<eq:imaginary>

== Formats
Like any math the notation conventions for quaternions varies. Within this paper the following
notations will be used:
$ q = (w, x, y, z) $
or
$ q = w + x i + y j + z k $

Its important to pay attention to the ordering, the, less popular, convention also exists to put the
real part last when listing the quaternion components: $q = (x, y, z, w)$. Additionally a common
convention used in many sources cited here is to write $q = [s, ab(v)]$ where $s$ is thre real
scalar component and $ab(v)$ is a vector defining the imaginary component. This can also be written
$q = [s, (x, y, z)]$. Unit quaternions are additionally frequently represented as an axis-angle
combination, appearing as $q = [cos(theta), sin(theta) ab(v)]$. The angle $theta$ however, is not
the angle of rotation that would result from @eq:rotation. This is discussed in detail in
@axisangle.

= Definitions
For the below definitions let $p,q in H$

== Identity
The identity element is defined to be@dam1998[lemma 1]:
$ I = (1, 0, 0, 0) $<eq:identity>

== Conjugate
The conjugate of quaternion $q$ is defined@naif2003:
$ q^* = (w, -x, -y, -z) $<eq:conjugate>

== Basic algebraic operations
=== Addition
The addition of quaternions $p$ and $q$ is simple combining like terms@eberly2002[eqn 1]:
$
  p + q & = (w_p + x_p i + y_p j + z_p k) + (w_q + x_q i + y_q j + z_q k) \
        & = w_p + w_q + (x_p + x_q) i + (y_p + y_q) j + (z_p + z_q) k \
        & = (w_p + w_q, x_p + x_q, y_p + y_q, z_p + z_q)
$<eq:add>

=== Multiplication
Using the definitions of the imaginary components in @eq:imaginary, multiplication of $p$ and $q$
results in@dantam2014[ax b]:
$
  p q & = (w_p + x_p i + y_p j + z_p k) (w_q + x_q i + y_q j + z_q k) \
      & = (w_p w_q - x_p x_q - y_p y_q - z_p z_q) \
      & + (w_p x_q + x_p w_q + y_p z_q - z_p y_q) i \
      & + (w_p y_q - x_p z_q + y_p w_q + z_p x_q) j \
      & + (w_p z_q + x_p y_q - y_p x_q + z_p w_q) k
$<eq:mul>
The corollary to this definition is that multiplicaiton of quaternions is not generally commutative.

=== Scalar multiplication
The multiplication of quaternion $q$ with scalar $r in RR$ is commutative and follows easily from
vector algrebra@dam1998[prop 6]:
$ r q = q r = (r w, r x, r y, r z) $<eq:smul>

=== Dot product
The dot product or inner product of $p$ and $q$ is@dam1998[def 9]:
$
  p dot q & = (w_p + x_p i + y_p j + z_p k) dot (w_q + x_q i + y_q j + z_q k) \
          & = w_p w_q + x_p x_q i + y_p y_q j + z_p z_q k \
          & = (w_p w_q, x_p x_q, y_p y_q, z_p z_q)
$<eq:dot>

== Other operations
=== Norm
The norm of a quaternion is conceptually similar to magnitude#footnote[There is some ambigiguity
  what is meant when referring to a norm. Some sources dont take the root, choosing instead to call
  that the modulus. This also would change @eq:inverse by making squaring the norm unnecessary.].
The canonical norm of $q$ is defined@dam1998[def 8]:
$ norm(q) = sqrt(w^2 + x^2 + y^2 + z^2) $<eq:norm>

=== Multiplicative inverse
The multiplicative inverse of quaternion $q$ is@dam1998[lemma 2]:
$ q^(-1) = 1 / q = q^* / norm(q)^2 $<eq:inverse>
Every quaternion has an inverse.

==== Division
Using the multiplicative inverse in @eq:inverse and multiplication from @eq:mul division follows
straightforwardly:
$
  p / q = p q^(-1)
$<eq:division>
Note that because multiplication is not commutative: $p q^(-1) = p / q != q^(-1) p$.

=== Exponential
To calculate the exponential of $q$, let $alpha = sqrt(x^2 + y^2 + z^2)$, the exponential is
then@sarkka2007[eqn 17]:
$
  exp(q) = e^q &= e^w (cos(alpha) + sin(alpha) x / alpha i + sin(alpha) y / alpha j + sin(alpha) z / alpha k) \
  &= (e^w cos(alpha), e^w sin(alpha) x / alpha, e^w sin(alpha) y / alpha, e^w sin(alpha) z / alpha)
$<eq:exponential>

=== Natural logarithm
Let $alpha = sqrt(x^2 + y^2 + z^2)$, the natural logarithm of $q$ is@sarkka2007[eqn 19]:
$
  ln(q) &= ln(norm(q)) + arccos(w / norm(q)) x / alpha i + arccos(w / norm(q)) y / alpha j + arccos(w / norm(q)) z / alpha k \
  &= (ln(norm(q)), arccos(w / norm(q)) x / alpha, arccos(w / norm(q)) y / alpha, arccos(w / norm(q)) z / alpha)
$<eq:log>

=== Power
Taking a quaternion power is defined as@sarkka2007[eqn 20]:
$ q^p = e^(ln(q) p) $<eq:power>

== Angular distance
The angular distance, in radians, between quaternions $p$ and $q$ can be found by letting
$d = q p^* = w + x i + y j + z k$ and then using#footnote[This formula comes from a #link(
    "https://stackoverflow.com/questions/23260939/distance-or-angular-magnitude-between-two-quaternions",
  )[stack overflow post] that cites a #link(
    "https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation#Recovering_the_axis-angle_representation",
  )[wikipedia article].
  // A derivation is in @ax:distance.
]#footnote[More information on the $arctan2$ function can be found here: #link(
    "https://en.wikipedia.org/wiki/Atan2",
  )]:
$ Phi = 2 arctan2(sqrt(x^2 + y^2 + z^2), w^2) $<eq:distance>

= Unit quaternions
A quaternion $q$ is said to be a unit or normalized quaternion if@dam1998[def 12]:
$ norm(q) = 1 $<eq:normcond>
The following properties hold for all unit quaternions:
$ norm(p q) = 1 $
$ q^(-1) = q^* $

== Choosing uniformly random unit quaternions
Shoemake's subgroup algorithm@shoemake1992[p 129] is an efficient way of generating random rotations
without rejection sampling. Let
$ u_1, u_2, u_3 in [0, 1] $
be independent variables randomly and uniformly distributed. Then take
$
  theta_1 & = 2 pi u_2 \
  theta_2 & = 2 pi u_3
$
and their sines and cosines
$
  s_1 & = sin(theta_1) \
  c_1 & = cos(theta_1) \
  s_2 & = sin(theta_2) \
  c_2 & = cos(theta_2)
$
Additionally compute
$
  r_1 & = sqrt(1 - u_0) \
  r_2 & = sqrt(u_0)
$
The final random unit quaternion is then:
$ q_"rand" = (s_1 r_1, c_1 r_1, s_2 r_2, c_2 r_2) $

= Quaternions for rotations
Unit quaternions cleanly represent a rotation in $RR^3$ while avoiding many of the problems with
Euler angles such as gimbal lock or numerical instability. Computing rotations using quaternions is
generally faster than using matracies. Any Cartesian reference frame can be rotated into any other
using a single quaternion, however $q$ and $-q$ will yield the same end result while taking
different paths.

== Rotating vectors
To rotate a vector $ab(v) in RR^3$ using a given unit quaternion $q in H_1$ the vector first needs
to be extended into a quaternion. An arbitrary real value can be chosen as $w$ for the new
quaternion $v = (0, bold(v)_1, bold(v)_2, bold(v)_3)$. Then rotation is a simple
formula@dam1998[prop 18]#footnote[Some sources use the inverse of q instead of its conjugate. It
  doesnt matter for unit quaternions but it should be investigated.]:
$ v_q = q v q^* $<eq:rotation>
The real component of $v_q$ can then be ignored. Quaternion rotation is composable, rotation by $p$
followed by rotation by $q$ is equivalent to rotation by $p q$@dam1998[prop 22].

== Axis and angle<axisangle>
A natural way of describing a rotation is with the the use of a rotation axis unit vector
$ab(v) = (x, y, z)$ and the angle to rotate $Phi$. Obtaining a quaternion, $q$ corresponding to this
rotation is a simple formula@naif2003:
$
  q & = cos(Phi / 2) - sin(Phi / 2) x i - sin(Phi / 2) y j - sin(Phi / 2) z k \
    & = (cos(Phi / 2), -sin(Phi / 2) x, -sin(Phi / 2) y, -sin(Phi / 2) z)
$<eq:axisangle>
Note that $Phi$ is halved as the rotation appearing in the $q = [cos(theta), sin(theta) ab(v)]$
notation for quaternions will result in a rotation of $2 theta$@dam1998[col 4].

Conversion from a quaternion back to an axis and angle is also straightforward:
$
    Phi & = 2 arctan2(sqrt(x^2 + y^2 + z^2), w^2) \
  ab(v) & = (x / sqrt(x^2 + y^2 + z^2), y / sqrt(x^2 + y^2 + z^2), z / sqrt(x^2 + y^2 + z^2))
$<eq:toaxisangle>
This formulation does have numerical instability when the imaginary components are small.

== Rotation matrix
A common way to represent rotations is through the use of a rotation matrix. In $RR^3$ this is a
$3 times 3$ or $4 times 4$ matrix calculated from either the Euler angles or from an axis and angle.
Quaternions can be robustly converted to and from the matrix equivalent. Algebraically working
$q v q^* = M v$ through to find the rotation matrix $M$ yields the following
relation@motekew2014[sec 3]@naif2003:
$
  M = mat(
    delim: "[",
    1 - 2 (y^2 + z^2), 2 (x y - w z), 2 (x z + w y), 0;
    2 (x y + w z), 1 - 2 (x^2 + z^2), 2 (y z - w x), 0;
    2 (x z - w y), 2 (y z + w x), 1 - 2 (x^2 + y^2), 0;
    0, 0, 0, 1;
  )
$<eq:tomatrix>
This relationship holds for all quaternions, however the matrix can be simplified using the
properties of unit quaternions in @eq:normcond along the diagnonal. For $q in H_1$ the equivalent
rotation matrix is@motekew2014[eqn 4]:
$
  M = mat(
    delim: "[",
    w^2 + x^2 - y^2 - z^2, 2 (x y - w z), 2 (x z + w y), 0;
    2 (x y + w z), w^2 - x^2 + y^2 - z^2, 2 (y z - w x), 0;
    2 (x z - w y), 2 (y z + w x), w^2 - x^2 - y^2 + z^2, 0;
    0, 0, 0, 1;
  )
$<eq:unitmatrix>

Converting from a rotation matrix to a quaternion is less straightforward.

Shepperd's algorithm@shepherd1978 is a common technique and guarantees a normalized quaternion so
long as the rotation matrix is precisely orthogonal. There are many improvements and optimizations
that have been shown in an attempt to reduce numerical instability
errors@wu2019@sarabandi2019@markley2008.
#todo

=== Shepperd's algorithm @shepherd1978
#todo

=== Bar-Itzhack's algorithm @baritzhack2000
#todo

=== Markley's algorithm @markley2008
$
  a & = cases(
        (1 + M_(11) - M_(22) - M_(33), M_(12) + M_(21), M_(13) + M_(31), M_(23) - M_(32)) & "if",
        (M_(21) + M_(12), 1 - M_(11) + M_(22) - M_(33), M_(23) + M_(32), M_(31) - M_(13)) & "if",
        (M_(31) + M_(13), M_(32) + M_(23), 1 - M_(11) - M_(22) + M_(33), M_(12) - M_(21)) & "if",
        (M_(23) - M_(32), M_(31) - M_(13), M_(12) - M_(21), 1 + M_(11) + M_(22) + M_(33)) & "if",
      ) \
  q & = a / norm(a)
$<eq:markley>
#todo

=== Sarabandi and Thomas' algorithm @sarabandi2019
$
  w & = cases(
        1 / 2 sqrt(1 + M_(11) + M_(22) + M_(33)) & "if" M_(11) + M_(22) + M_(33) > eta,
        1 / 2 sqrt(((M_(32) - M_(23))^2 + (M_(13) - M_(31))^2 + (M_(21) - M_(12))^2) / (3 - M_(11) - M_(22) - M_(33))) & "otherwise"
      ) \
  x & = cases(
        1 / 2 sqrt(1 + M_(11) - M_(22) - M_(33)) & "if" M_(11) - M_(22) - M_(33) > eta,
        1 / 2 sqrt(((M_(32) - M_(23))^2 + (M_(12) + M_(21))^2 + (M_(31) + M_(13))^2) / (3 - M_(11) + M_(22) + M_(33))) & "otherwise"
      ) \
  y & = cases(
        1 / 2 sqrt(1 - M_(11) + M_(22) - M_(33)) & "if" -M_(11) + M_(22) - M_(33) > eta,
        1 / 2 sqrt(((M_(13) - M_(31))^2 + (M_(12) + M_(21))^2 + (M_(23) + M_(32))^2) / (3 + M_(11) - M_(22) + M_(33))) & "otherwise"
      ) \
  z & = cases(
        1 / 2 sqrt(1 - M_(11) - M_(22) + M_(33)) & "if" -M_(11) - M_(22) + M_(33) > eta,
        1 / 2 sqrt(((M_(21) - M_(12))^2 + (M_(31) + M_(13))^2 + (M_(32) + M_(23))^2) / (3 + M_(11) + M_(22) - M_(33))) & "otherwise"
      ) \
  q & = w + sgn(M_(32) - M_(23)) x i + sgn(M_(13) - M_(31)) y j + sgn(M_(21) - M_(12)) z k
$<eq:sarabandi>
A suitable $eta$ value
#todo

=== Wu's algorithm @wu2019
#todo

== Interpolation
=== Linear interpolation
Linear interpolation between orientations takes a straight path in quaternion space. This results in
the angular velocity speeding up near the middle of travel@dam1998[sec 6.1.3]. The quaternion lerp
between $p$ and $q$ given parameter $t in [0, 1]$can be stated:
$
  lerp(p, q, t) = p(1 - t) + q t
$<eq:lerp>

=== Spherical interpolation
The spherical linear interpolation or slerp between two quaternion rotations has constant angular
velocity and follows the great circle path between the two orientations@shoemake1985. There are
several equivalent ways to define a slerp between $p$ and $q$ given parameter
$t in [0, 1]$@dam1998[sec 6.1.5]:
$
  slerp(p, q, t) & = p (p^* q)^t \
                 & = (p q^*)^(1-t) q \
                 & = (q p^*)^t p \
                 & = q (q^* p)^(1 - t)
$<eq:slerp>

// #show: appendix
// = Proofs
// Im no expert on mathematical rigor. What is written in this section is my best attempt and probably
// closer to a worked example than proof.
//
// == Proof of @eq:distance<ax:distance>
// Let $ab(v)_1, ab(v)_2 in RR^3$
