import type { ReactElement } from "react";
import AddPlayer from "./components/AddPlayer";
import PlayerList from "./components/PlayerList";
import { PlayersProvider } from "./context/playersContext";

const App = (): ReactElement => {
  return (
    <div>
      <h1>💯 Player scoreboard</h1>
      <hr />
      <PlayersProvider>
        <AddPlayer />
        <PlayerList />
      </PlayersProvider>
    </div>
  );
};

export default App;
