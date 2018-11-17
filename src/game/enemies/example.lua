return {
  name = "Example",
  id = "example",
  fights = "BONES",

  default_state = "IDLE",
  states = {

    IDLE = {
      frames = 120,
      vulnerability = {
        left = "WHIFF",
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
      on_getting_hit = "IDLE",
      on_block = "IDLE",
      on_parry = "IDLE",
      on_end = "ATTACK",
    },

    ATTACK = {
      frames = 10,
      vulnerability = {
        left = "WHIFF",
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
      on_getting_hit = "IDLE",
      on_block = "IDLE",
      on_parry = "IDLE",
      on_end = "IDLE",
    }

  }
}
