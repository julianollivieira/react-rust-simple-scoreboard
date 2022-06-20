import { ReactElement } from "react";
import Player from "../types/player";
import { usePlayers } from "../context/playersContext";

interface Props {
  player: Player;
}

const PlayerItem = ({ player }: Props): ReactElement => {
  const { setPlayerScore } = usePlayers();

  const increment = (player: Player) => {
    setPlayerScore(player, player.score + 1);
  };

  const decrement = (player: Player) => {
    setPlayerScore(player, player.score - 1);
  };

  return (
    <div>
      <details>
        <summary>
          🧑 {player.name}, {player.score} points
        </summary>
        <pre>
          <code>UUID: {player.id}</code>
        </pre>
        <h3>Score</h3>
        <div style={{ display: "flex" }}>
          <button onClick={() => decrement(player)}>-1</button>
          <button onClick={() => increment(player)}>+1</button>
        </div>
      </details>
    </div>
  );
};

export default PlayerItem;
