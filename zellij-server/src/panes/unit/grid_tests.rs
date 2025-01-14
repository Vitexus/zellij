use super::super::Grid;
use ::insta::assert_snapshot;
use zellij_utils::{vte, zellij_tile::data::Palette};

fn read_fixture(fixture_name: &str) -> Vec<u8> {
    let mut path_to_file = std::path::PathBuf::new();
    path_to_file.push("../src");
    path_to_file.push("tests");
    path_to_file.push("fixtures");
    path_to_file.push(fixture_name);
    let content = std::fs::read(path_to_file)
        .unwrap_or_else(|_| panic!("could not read fixture {:?}", &fixture_name));
    content
}

#[test]
fn vttest1_0() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest1-0";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest1_1() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest1-1";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest1_2() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest1-2";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest1_3() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest1-3";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest1_4() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest1-4";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest1_5() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest1-5";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_0() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-0";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_1() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-1";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_2() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-2";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_3() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-3";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_4() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-4";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_5() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-5";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_6() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-6";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_7() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-7";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_8() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-8";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_9() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-9";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_10() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-10";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_11() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-11";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_12() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-12";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_13() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-13";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest2_14() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest2-14";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest3_0() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(41, 110, Palette::default());
    let fixture_name = "vttest3-0";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest8_0() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(51, 97, Palette::default());
    let fixture_name = "vttest8-0";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest8_1() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(51, 97, Palette::default());
    let fixture_name = "vttest8-1";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest8_2() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(51, 97, Palette::default());
    let fixture_name = "vttest8-2";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest8_3() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(51, 97, Palette::default());
    let fixture_name = "vttest8-3";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest8_4() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(51, 97, Palette::default());
    let fixture_name = "vttest8-4";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn vttest8_5() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(51, 97, Palette::default());
    let fixture_name = "vttest8-5";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn csi_b() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(51, 97, Palette::default());
    let fixture_name = "csi-b";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn csi_capital_i() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(51, 97, Palette::default());
    let fixture_name = "csi-capital-i";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn csi_capital_z() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(51, 97, Palette::default());
    let fixture_name = "csi-capital-z";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn terminal_reports() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(51, 97, Palette::default());
    let fixture_name = "terminal_reports";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid.pending_messages_to_pty));
}

#[test]
fn wide_characters() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 104, Palette::default());
    let fixture_name = "wide_characters";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn wide_characters_line_wrap() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 104, Palette::default());
    let fixture_name = "wide_characters_line_wrap";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn fish_wide_characters_override_clock() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 104, Palette::default());
    let fixture_name = "fish_wide_characters_override_clock";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn bash_delete_wide_characters() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 104, Palette::default());
    let fixture_name = "bash_delete_wide_characters";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn delete_wide_characters_before_cursor() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 104, Palette::default());
    let fixture_name = "delete_wide_characters_before_cursor";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn delete_wide_characters_before_cursor_when_cursor_is_on_wide_character() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 104, Palette::default());
    let fixture_name = "delete_wide_characters_before_cursor_when_cursor_is_on_wide_character";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn delete_wide_character_under_cursor() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 104, Palette::default());
    let fixture_name = "delete_wide_character_under_cursor";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn replace_wide_character_under_cursor() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 104, Palette::default());
    let fixture_name = "replace_wide_character_under_cursor";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn wrap_wide_characters() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 90, Palette::default());
    let fixture_name = "wide_characters_full";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn wrap_wide_characters_on_size_change() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 93, Palette::default());
    let fixture_name = "wide_characters_full";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    grid.change_size(21, 90);
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn unwrap_wide_characters_on_size_change() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 93, Palette::default());
    let fixture_name = "wide_characters_full";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    grid.change_size(21, 90);
    grid.change_size(21, 93);
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn wrap_wide_characters_in_the_middle_of_the_line() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 91, Palette::default());
    let fixture_name = "wide_characters_line_middle";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}

#[test]
fn wrap_wide_characters_at_the_end_of_the_line() {
    let mut vte_parser = vte::Parser::new();
    let mut grid = Grid::new(21, 90, Palette::default());
    let fixture_name = "wide_characters_line_end";
    let content = read_fixture(fixture_name);
    for byte in content {
        vte_parser.advance(&mut grid, byte);
    }
    assert_snapshot!(format!("{:?}", grid));
}
