# frozen_string_literal: true

module MicroMcp
  module Schema
    # Schema builder class for chaining
    class SchemaBuilder
      def initialize(schema)
        @schema = schema
      end

      def required
        @schema.merge(required: true)
      end

      def optional
        @schema
      end

      def method_missing(method, *args)
        if @schema.respond_to?(method)
          @schema.send(method, *args)
        else
          super
        end
      end

      def respond_to_missing?(method, include_private = false)
        @schema.respond_to?(method) || super
      end
    end

    # Alternative builder for required-first syntax
    class RequiredBuilder
      def integer(description = nil)
        schema = {type: "integer"}
        schema[:description] = description if description
        schema.merge(required: true)
      end

      def string(description = nil)
        schema = {type: "string"}
        schema[:description] = description if description
        schema.merge(required: true)
      end

      def number(description = nil)
        schema = {type: "number"}
        schema[:description] = description if description
        schema.merge(required: true)
      end

      def boolean(description = nil)
        schema = {type: "boolean"}
        schema[:description] = description if description
        schema.merge(required: true)
      end

      def array(items_type = nil, description = nil)
        schema = {type: "array"}
        schema[:items] = items_type if items_type
        schema[:description] = description if description
        schema.merge(required: true)
      end
    end

    # Helper methods for common schema patterns with builder support
    def self.integer(description = nil)
      schema = {type: "integer"}
      schema[:description] = description if description
      SchemaBuilder.new(schema)
    end

    def self.string(description = nil)
      schema = {type: "string"}
      schema[:description] = description if description
      SchemaBuilder.new(schema)
    end

    def self.number(description = nil)
      schema = {type: "number"}
      schema[:description] = description if description
      SchemaBuilder.new(schema)
    end

    def self.boolean(description = nil)
      schema = {type: "boolean"}
      schema[:description] = description if description
      SchemaBuilder.new(schema)
    end

    def self.array(items_type = nil, description = nil)
      schema = {type: "array"}
      schema[:items] = items_type if items_type
      schema[:description] = description if description
      SchemaBuilder.new(schema)
    end

    # Create object schema with properties and required fields
    def self.object(**properties)
      required_fields = []
      schema_properties = {}

      properties.each do |key, value|
        if value.is_a?(Hash) && value[:required] == true
          required_fields << key.to_s
          value = value.dup
          value.delete(:required)
        end
        schema_properties[key] = value
      end

      schema = {
        type: "object",
        properties: schema_properties
      }
      schema[:required] = required_fields unless required_fields.empty?
      schema
    end

    # Entry point for required-first syntax
    def self.required
      RequiredBuilder.new
    end
  end
end
