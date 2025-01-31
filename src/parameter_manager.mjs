export class ParameterManager {
  constructor() {
    this.parameters = {
      id1: 100,
      id2: 200,
      id3: 300,
    };
  }

  getValue(id) {
    return this.parameters[id];
  }

  setValue(id, value) {
    this.parameters[id] = value;
  }
}
