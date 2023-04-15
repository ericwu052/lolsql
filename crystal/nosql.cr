require "socket"

class Server
  def self.start
    server = TCPServer.new("localhost", 5000)
    server.recv_buffer_size = 4096
    data = Hash(String, String).new

    loop do
      socket = server.accept
      if socket
        spawn do
          loop do
            if request = socket.gets
              request = request.split(" ").map{|item| item.strip }
              command = request[0]
              key = request[1]

              if command == "set"
                value = request[2]

                data[key] = value

                socket.puts(value)
              elsif command == "get"
                value = data[key]

                socket.puts(value)
              else
                socket.puts("error: #{command} is not a valid command")
              end
            end
          end
        end
      end
    end
  end
end
