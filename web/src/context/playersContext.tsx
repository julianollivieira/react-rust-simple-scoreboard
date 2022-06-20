import type Player from "../types/player";
import { createContext, useContext, useEffect, useState } from "react";
import type { ReactNode } from "react";
import { getPlayers } from "../services/players";
import { setPlayerScore as setPlayerScoreAction } from "../services/players";

interface Context {
  players: Player[];
  setPlayerScore: (player: Player, score: number) => void;
}

const PlayersContext = createContext<Context>({
  players: [],
  setPlayerScore: () => {},
});

const usePlayers = () => useContext(PlayersContext);

const PlayersProvider = ({ children }: { children: ReactNode }) => {
  const [players, setPlayers] = useState<Context["players"]>([]);

  useEffect(() => {
    getPlayers().then((newPlayers) => {
      setPlayers(newPlayers);
    });
  }, []);

  const setPlayerScore = (player: Player, score: number) => {
    setPlayerScoreAction(player, score).then((newPlayer) => {
      setPlayers((prevPlayers) => {
        const newPlayers = [...prevPlayers];
        const index = newPlayers.findIndex((p) => p.id === newPlayer.id);
        newPlayers[index] = newPlayer;
        return newPlayers;
      });
    });
  };

  return (
    <PlayersContext.Provider value={{ players, setPlayerScore }}>
      {children}
    </PlayersContext.Provider>
  );
};

export { PlayersProvider, usePlayers };
