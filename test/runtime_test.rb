require "test_helper"

class RuntimeTest < Minitest::Test
  def test_sample_delegates_to_native
    runtime = MicroMcp::Runtime.allocate
    called = false
    runtime.define_singleton_method(:__native_sample!) do |json|
      called = true
      json
    end
    runtime.define_singleton_method(:sampling_supported?) { true }

    result = runtime.sample!(foo: "bar")

    assert_equal({"foo" => "bar"}, result)
    assert called
  end
end
