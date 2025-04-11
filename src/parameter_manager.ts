export class ParameterManager {    
    parameters: { id1: number; id2: number; id3: number; };
    constructor() {
      this.parameters = {
        id1: 100,
        id2: 200,
        id3: 300,
      }
    }

  getValueFromId(id: string) {
    return this.parameters[id];
  }

  setValue(id: string, value:number) {
    this.parameters[id] = value;
  }

  updateAll() {
    for (const key in this.parameters) {
      this.parameters[key as keyof typeof this.parameters] += 1;
    }
  }

  getTableData() {
    return {
      ids: Object.keys(this.parameters),
      values: Object.values(this.parameters)
    }
  }
}
