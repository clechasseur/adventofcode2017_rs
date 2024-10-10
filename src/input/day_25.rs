pub const STARTING_STATE: &str = "A";
pub const CHECKSUM_AFTER: usize = 12_656_374;
pub const STATES: &str = r#"
  {
    "A": [
      {
        "value": 1,
        "movement": "right",
        "next_state": "B"
      },
      {
        "value": 0,
        "movement": "left",
        "next_state": "C"
      }
    ],

    "B": [
      {
        "value": 1,
        "movement": "left",
        "next_state": "A"
      },
      {
        "value": 1,
        "movement": "left",
        "next_state": "D"
      }
    ],

    "C": [
      {
        "value": 1,
        "movement": "right",
        "next_state": "D"
      },
      {
        "value": 0,
        "movement": "right",
        "next_state": "C"
      }
    ],

    "D": [
      {
        "value": 0,
        "movement": "left",
        "next_state": "B"
      },
      {
        "value": 0,
        "movement": "right",
        "next_state": "E"
      }
    ],

    "E": [
      {
        "value": 1,
        "movement": "right",
        "next_state": "C"
      },
      {
        "value": 1,
        "movement": "left",
        "next_state": "F"
      }
    ],

    "F": [
      {
        "value": 1,
        "movement": "left",
        "next_state": "E"
      },
      {
        "value": 1,
        "movement": "right",
        "next_state": "A"
      }
    ]
  }
"#;
