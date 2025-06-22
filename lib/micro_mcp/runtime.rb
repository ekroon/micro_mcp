# frozen_string_literal: true

require "json"

module MicroMcp
  class Runtime < MicroMcpNative::Runtime
    def sample!(params)
      raise "Client does not support sampling" unless sampling_supported?

      json = params.to_json
      result = super(json)
      JSON.parse(result)
    end
  end
end
