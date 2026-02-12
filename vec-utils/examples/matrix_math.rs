use vec_utils::matrix::real::Matrix2x2;

fn main() {
    let a = Matrix2x2::from_nested_arr([[1.0, 2.0], [3.0, 4.0]]);
    let b = Matrix2x2::from_nested_arr([[5.0, 6.0], [7.0, 8.0]]);

    println!("--- Step 1: Initial Matrices ---");
    println!("Matrix A:\n{}", a);
    println!("Matrix B:\n{}", b);

    println!("\n--- Step 2: Multiplication (A * B) ---");
    println!("Calculating dot products...");
    let result = a * b;
    println!("Resulting Matrix:\n{}", result);

    println!("\n--- Step 3: Taking the Adjoint of A ---");
    let adj_a = a.adjoint();
    println!("Cofactors calculated and transposed...");
    println!("Adjoint Matrix of A:\n{}", adj_a);

    println!("\n--- Step 4: Verification ---");
    let det_a = a.determinant();
    println!("Determinant of A: {}", det_a);

    let verification = a * adj_a;
    println!("A * Adjoint(A):\n{}", verification);
    println!(
        "(Should be a diagonal matrix of [{}, 0, 0, {}])",
        det_a, det_a
    );
}
