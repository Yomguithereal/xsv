use workdir::Workdir;

#[test]
fn enumerate() {
    let wrk = Workdir::new("enumerate");
    wrk.create("data.csv", vec![
        svec!["letter", "number"],
        svec!["a", "13"],
        svec!["b", "24"],
        svec!["c", "72"],
        svec!["d", "7"],
    ]);
    let mut cmd = wrk.command("enumerate");
    cmd.arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "index"],
        svec!["a", "13", "0"],
        svec!["b", "24", "1"],
        svec!["c", "72", "2"],
        svec!["d", "7", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn enumerate_column_name() {
    let wrk = Workdir::new("enumerate");
    wrk.create("data.csv", vec![
        svec!["letter", "number"],
        svec!["a", "13"],
        svec!["b", "24"],
        svec!["c", "72"],
        svec!["d", "7"],
    ]);
    let mut cmd = wrk.command("enumerate");
    cmd.arg("--column-name").arg("row").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "row"],
        svec!["a", "13", "0"],
        svec!["b", "24", "1"],
        svec!["c", "72", "2"],
        svec!["d", "7", "3"],
    ];
    assert_eq!(got, expected);
}

