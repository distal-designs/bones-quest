return {
  name = "Example",
  id = "example",
  fights = "BONES",

  default_state = "IDLE",
  states = {

    IDLE = {
      frames = 10,
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
      on_end = function (world)
        if math.random(100) > 30 then
          return "IDLE"
        else
          return "IDLE"
        end
      end
    }

  }
}
