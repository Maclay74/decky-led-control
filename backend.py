import socket

class Backend:
    def __init__(self, ip, port):
        self.ip = ip
        self.port = port

    def get_color(self):
        return self.send_command("get_color")  
    
    def set_color(self, color):
        command = f"set_color {color[0]} {color[1]} {color[2]}"
        return self.send_command(command)  
    
    def set_color(self):
        return self.send_command("turn_off")  

    def send_command(self, command) -> str:
        try:
            # Create a TCP socket
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            
            # Connect to the server
            server_address = (self.ip, self.port)
            sock.connect(server_address)
            
            # Send the command to the server
            sock.sendall(command.encode('utf-8')) # FIX THIS encode is not right here

            # Receive the response from the server
            response = sock.recv(1024).decode()
            print(f"Received response: {response}")
            return response

        except Exception as e:
            return f"Error {e}"

        finally:
            # Close the socket
            sock.close()