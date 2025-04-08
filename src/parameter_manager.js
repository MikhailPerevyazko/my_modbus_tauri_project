export class ParameterManager {
  constructor() {
    this.parameters = {
      id1: 100,
      id2: 200,
      id3: 300,
      id4: 400,
      id5: 500,
      id6: 600,
    };
  }

  getVal(id) {
    return this.parameters[id];
  }

  setVal(id, value) {
    this.parameters[id] = value;
  }

  getAllIdes() {
    return Object.keys(this.parameters);
  }

  getAllValues() {
    return Object.values(this.parameters);
  }
}
