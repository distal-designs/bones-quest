local is_plain_table = require 'src.util.lua.is-plain-table'

local extend
extend = function (base, ext)
  for k, v in pairs(ext) do
    if is_plain_table(base[k]) and is_plain_table(v) then
      extend(base[k], v)
    else
      base[k] = v
    end
  end
  return base
end

return extend
