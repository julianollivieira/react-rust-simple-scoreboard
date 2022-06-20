import { ReactElement } from "react";
import { usePlayers } from "../context/playersContext";
import PlayerItem from "./PlayerItem";

const PlayerList = (): ReactElement => {
  const { players } = usePlayers();

  return (
    <>
      <h3>List of players</h3>
      <div>
        {players.map((player) => (
          <PlayerItem key={player.id} player={player} />
        ))}
      </div>
    </>
  );
};

export default PlayerList;
