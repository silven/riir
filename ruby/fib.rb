#!/usr/bin/env ruby

require "ffi"
require "benchmark"

include Benchmark

class Fib
  module Rust
    extend FFI::Library
    ffi_lib "./target/release/librust_fib.so"

    attach_function :fib, [:int], :size_t
  end

  def self.ruby_fib(n, prev_fib = 0, next_fib = 1)
    if n <= 1
      prev_fib + next_fib
    else
      ruby_fib(n - 1, next_fib, prev_fib + next_fib)
    end
  end

  def self.rust_fib(n)
    Fib::Rust::fib(n)
  end
end

print (0..20).map { |n| Fib.ruby_fib(n) }
puts
print (0..20).map { |n| Fib.rust_fib(n) }
puts

Benchmark.benchmark(CAPTION, 10, FORMAT) do |x|
    puts "Comparing Fibs"
    x.report("Ruby to 100: ") { (0..100).each { |n| Fib.ruby_fib(n) } }
    x.report("Rust to 100: ") { (0..100).each { |n| Fib.rust_fib(n) } }
end