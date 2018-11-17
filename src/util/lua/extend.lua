local is_plain_table = require 'src.util.lua.is-plain-table'

local extend
extend = function (base, ext)
  local new = {}
  for k, v in pairs(base) do
    new[k] = v
  end

  for k, v in pairs(ext) do
    if is_plain_table(base[k]) and is_plain_table(v) then
      new[k] = extend(base[k], v)
    else
      new[k] = v
    end
  end
  return new
end

return extend
