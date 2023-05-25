import os, sys
import pathlib
import subprocess
import asyncio
import decky_plugin

sys.path.append(os.path.dirname(__file__))

from backend import Backend

HOST = '127.0.0.1'
PORT = 32723
HOME_DIR = str(pathlib.Path(os.getcwd()).parent.parent.resolve())
PARENT_DIR = str(pathlib.Path(__file__).parent.resolve())

class Plugin:
    
    backend_proc = None
    backend: Backend = None

    async def _main(self):

        try:
            env_proc = dict(os.environ)
            if "LD_LIBRARY_PATH" in env_proc:
                env_proc["LD_LIBRARY_PATH"] += ":" + PARENT_DIR + "/bin"
            else:
                env_proc["LD_LIBRARY_PATH"] = ":" + PARENT_DIR + "/bin"

            # command = ["sudo", "-S", PARENT_DIR + "/bin/backend", HOST, str(PORT)]
            # backend_proc = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            # backend_proc.communicate("gamer".encode())

        except Exception as e: 
            decky_plugin.logger.info(f"Error {e}")
            return
        
        await asyncio.sleep(1)
        self.backend = Backend(HOST, PORT)

        # while True:
        #     await asyncio.sleep(1)

    async def _unload(self):
        if self.backend_proc is not None:
            self.backend_proc.terminate()
            try:
                self.backend_proc.wait(timeout=5) # 5 seconds timeout
            except subprocess.TimeoutExpired:
                self.backend_proc.kill()
            self.backend_proc = None

    async def get_style(self):
        response = self.backend.get_color()
        return response
    
    async def set_color(self, color):
        response = self.backend.set_color(color)
        return response

    # Migrations that should be performed before entering `_main()`.
    async def _migration(self):
        pass
