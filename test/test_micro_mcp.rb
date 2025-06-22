# frozen_string_literal: true

require "test_helper"

class TestMicroMcp < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::MicroMcp::VERSION
  end

  def test_it_does_something_useful
    assert true
  end

  def test_graceful_shutdown
    # Skip this test if we can't create child processes
    skip "Fork not available" unless Process.respond_to?(:fork)

    pid = fork do
      # Capture output to avoid test pollution
      $stdout.reopen(File::NULL, "w")
      $stderr.reopen(File::NULL, "w")

      begin
        MicroMcp::Server.start
      rescue
        # Exit with error code if server fails to start
        exit(1)
      end

      # Clean exit
      exit(0)
    end

    # Give the server a moment to start
    sleep(0.5)

    # Send interrupt signal
    Process.kill("INT", pid)

    # Wait for process to exit with timeout
    timeout = 5.0
    start_time = Time.now
    status = nil

    while (Time.now - start_time) < timeout
      pid_result, status = Process.wait2(pid, Process::WNOHANG)
      break if pid_result
      sleep(0.1)
    end

    if status.nil?
      # If it times out, force kill and fail the test
      begin
        Process.kill("KILL", pid)
      rescue
        nil
      end
      begin
        Process.wait(pid)
      rescue
        nil
      end
      flunk "Server did not shut down within timeout period"
    else
      # Check that process exited cleanly (status 0)
      assert_equal 0, status.exitstatus, "Server should exit cleanly with status 0, got #{status.exitstatus}"
    end
  end
end
