import { defineStore } from "pinia";

export const useOverlayStore = defineStore({
  id: "overlay",
  state: () => ({
    overlay: "none",
    overlayData: {},
  }),
  actions: {
    selectOverlay(type) {
      this.overlay = type;
      this.overlayData = {};
      if (type != "none") {
        fetch("https://api.openlineplanner.xatellite.io/overlay", {
          method: "POST",
          body: JSON.stringify({ layer: type }),
          headers: {
            "Content-type": "application/json",
          },
        })
          .then((data) => data.json())
          .then((overlayData) => {
            this.overlayData = JSON.parse(overlayData.layerGeoJson);
          });
      }
    },
  },
});
