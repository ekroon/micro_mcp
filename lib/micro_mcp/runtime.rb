# frozen_string_literal: true

require "json"

module MicroMcpNative
  class Runtime
    alias_method :__native_sample!, :sample!

    def sample!(params)
      raise "Client does not support sampling" unless sampling_supported?

      json = params.to_json
      result = __native_sample!(json)
      JSON.parse(result)
    end
  end
end

module MicroMcp
  Runtime = MicroMcpNative::Runtime
end
