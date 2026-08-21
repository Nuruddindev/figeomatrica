name: "Geometrize a figure"
description: "Claim a figure and contribute its geometry spec + examples"
labels: ["contributing"]
body:
  - type: markdown
    attributes:
      value: |
        Terima kasih mau berkontribusi! Alur singkat:
        1. Pilih satu figur dari `data/figures/` yang `"geometri": null`.
        2. Komentar di issue ini: *"Saya mengerjakan `<nama-file>.json`"* agar tidak ada kerja ganda.
        3. Isi `geometri` + `contoh` + `atribusi`, buka PR — CI akan memverifikasi mesin secara otomatis.
        Panduan lengkap: [CONTRIBUTING.md](../blob/main/CONTRIBUTING.md)
  - type: input
    id: figure
    attributes:
      label: "File figur yang dikerjakan"
      description: "mis. data/figures/epizeuxis.json"
      placeholder: "data/figures/....json"
    validations:
      required: true
  - type: checkboxes
    id: checklist
    attributes:
      label: "Checklist"
      options:
        - label: "Figur ini belum memiliki geometri (belum diklaim orang lain)"
        - label: "Saya mengisi bentuk kanonik: operasi × jangkar × satuan × pengulangan (+ template bila relevan)"
        - label: "Saya menyertakan contoh positif yang pasti memicu pola dan contoh negatif yang pasti tidak"
        - label: "Definisi prosa TIDAK saya salin dari sumber berhak cipta (Silva Rhetoricae dsb.)"
