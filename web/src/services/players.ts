import Player from "../types/player";

const getPlayers = async (): Promise<Player[]> => {
  const response = await fetch("http://localhost:2000/players");
  return await response.json();
};

const setPlayerScore = async (
  player: Player,
  score: number
): Promise<Player> => {
  const response = await fetch(`http://localhost:2000/players/${player.id}`, {
    method: "PATCH",
    body: JSON.stringify({ name: player.name, score }),
    headers: {
      "Content-Type": "application/json",
    },
  });
  return await response.json();
};

export { getPlayers, setPlayerScore };
