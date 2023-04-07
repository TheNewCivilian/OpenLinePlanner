import { defineStore } from "pinia";
import { getMethods, getAdminBounds, getLayers } from "../helpers/api";

export const useDataStore = () => {
  const innerStore = defineStore({
    id: "data",
    state: () => ({
      methods: null,
      addPoint: null,
      areas: [],
      noAreas: false,
      selectedMethod: null,
      selectedArea: null,
      mapBounds: [],
      mapHighligh: null,
      loadingLayer: false,
    }),
    getters: {
      questionsResolved(state) {
        if (state.selectedMethod) {
          for (const index in state.selectedMethod.questions) {
            const question = state.selectedMethod.questions[index];
            if (!question.answer) return false;
          }
          return true;
        }
        return false;
      },
    },
    actions: {
      setAddPoint(value) {
        this.addPoint = value;
        this.resetSelection();
        getAdminBounds(value)
          .then((areas) => {
            this.areas = areas.features;
            this.mapBounds = areas.features;
            this.mapHighligh = null;
            if (areas.length == 0) {
              this.noAreas = true;
            }
          })
          .catch(() => {
            console.log("error");
            this.noAreas = true;
          });
      },
      removeAddPoint() {
        this.addPoint = null;
      },
      highlightArea(area) {
        this.mapHighligh = area;
      },
      selectMethod(method) {
        // ToDo: Fix this workaround..?!
        this.selectedMethod = JSON.parse(JSON.stringify(method));
      },
      selectArea(area) {
        this.selectedArea = area;
      },
      answerQuestion(index, answer) {
        this.selectedMethod.questions[index].answer = answer;
      },
      async loadMethods() {
        this.methods = await getMethods();
      },
      async loadLayers() {
        this.layers = await getLayers();
      },
      resetSelection() {
        this.selectedArea = null;
        this.selectedMethod = null;
        this.noAreas = false;
        this.areas = [];
      },
    },
  });
  // Async Init
  const s = innerStore();
  s.loadMethods();
  s.loadLayers();
  return s;
};
