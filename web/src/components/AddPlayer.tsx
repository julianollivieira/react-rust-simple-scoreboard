import type { ReactElement, SyntheticEvent } from "react";

const AddPlayer = (): ReactElement => {
  const handleSubmit = (e: SyntheticEvent) => {
    e.preventDefault();
    // TODO
  };

  return (
    <>
      <h3>Add a player</h3>
      <form onSubmit={handleSubmit} style={{ display: "flex" }}>
        <input type="text" placeholder="Player name" />
        <input type="number" placeholder="Points" />
        <button type="submit">➕ Add Player</button>
      </form>
    </>
  );
};

export default AddPlayer;
