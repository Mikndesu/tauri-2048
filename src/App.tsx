import "./App.css";

const tile_array = (() => {
  let arr: number[][] = [];
  Array(4).fill(0).map((_, i1) => {
    Array(4).fill(0).map((_, i2) => {
      arr.push([i1, i2]);
    })
  })
  return arr;
})();

function App() {
  return (
    <div className="container">
      <div className="container-game">
        {
          <div>
            {
              tile_array.map((_, __, val) => {
                return (
                  <div>

                  </div>
                )
              })
            }
          </div>
        }
      </div>
    </div>
  );
}

export default App;
