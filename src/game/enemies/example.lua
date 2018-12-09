return {
  name = "Example",
  id = "example",
  fights = "BONES",

  initial_state = "IDLE",
  states = {

    IDLE = {
      frames = 120,
      vulnerability = {
        left = "HIT",
        right = "BLOCK",
        parry = false,
      },
      hitzones = {
        left = false,
        right = false,
        duck = false,
        stand = false,
      },
      after_hitting_player = "IDLE",
      on_getting_hit = "PUNCHED",
      on_block = "IDLE",
      on_parry = "IDLE",
      on_end = "ATTACK",
    },

    ATTACK = {
      frames = 10,
      vulnerability = {
        left = "HIT",
        right = "BLOCK",
        parry = false,
      },
      hitzones = {
        left = true,
        right = false,
        duck = false,
        stand = false,
      },
      after_hitting_player = "IDLE",
      on_getting_hit = "PUNCHED",
      on_block = "IDLE",
      on_parry = "IDLE",
      on_end = "IDLE",
    },

    PUNCHED = {
      frames = 30,
      vulnerability = {
        left = "WHIFF",
        right = "WHIFF",
        parry = false,
      },
      hitzones = {
        left = false,
        right = false,
        duck = false,
        stand = false,
      },
      after_hitting_player = "IDLE",
      on_getting_hit = "IDLE",
      on_block = "IDLE",
      on_parry = "IDLE",
      on_end = "IDLE",
    },

  }
}
