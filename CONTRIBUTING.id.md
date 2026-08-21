*[English version](CONTRIBUTING.md) · Versi Indonesia*

# Contributing to Figeometrica

Terima kasih! Panduan ini menjelaskan cara menyumbang **geometri figur** —
spesifikasi terstruktur yang membuat sebuah figur retoris dapat dideteksi
mesin. Anda tidak perlu menulis kode; satu file JSON + contoh kalimat sudah
cukup.

## Tesis dalam satu baris

> Setiap figur, bila didefinisikan dengan baik, adalah operasi atas deret:
> **operasi × jangkar × satuan × pengulangan**.

## Bentuk kanonik

| Field | Isi | Nilai yang sah |
|---|---|---|
| `jangkar` | di mana pola menempel | `Awal`, `Akhir`, `Sisipan`, `UnitUtuh`, `AntarUnit` |
| `kelas` | kelas kesetaraan elemen | `Leksikal`, `Akar`, `Gramatikal`, `Konseptual` |
| `satuan` | satuan elemen yang dioperasikan | `grafem`, `kata`, `frasa`, `unit`, `wacana` |
| `operasi` | operasi klasik (operae) | `adjectio`, `detractio`, `immutatio`, `transmutatio`, `repetitio` |
| `minim_ulangan` | ulangan minimum pola repetisi | angka ≥ 1 |
| `template` | slot pola (opsional) | `["A","*","A"]` — id sama = label sama, `*` = apa pun |
| `catatan` | penjelasan singkat (opsional) | teks bebas |

Contoh entry lengkap (`data/figures/anaphora.json`):

```json
{
  "id": 59,
  "name": "anaphora",
  "categories": ["of Repetition"],
  "geometri": {
    "jangkar": "Awal", "kelas": "Leksikal", "satuan": "unit",
    "operasi": "repetitio", "minim_ulangan": 2,
    "template": ["A", "*", "A"]
  },
  "contoh": {
    "positif": [["We came.", "We saw.", "We conquered."]],
    "negatif": [["He came.", "They saw.", "It ended."]]
  },
  "atribusi": { "geometri": "username-anda", "contoh": "username-anda", "lisensi": "MIT" }
}
```

## Aturan contoh (bagian terpenting)

- **Positif** HARUS memicu pola; **negatif** HARUS TIDAK memicu (mirip tapi
  bukan figur itu).
- Tiap contoh = array unit wacana (kalimat). Figur lintas-unit (anaphora,
  anadiplosis, climax) butuh beberapa unit; antimetabole cukup satu.
- Validator menjalankan matcher deterministik atas contoh Anda. Lulus =
  kontribusi Anda tervalidasi mesin; gagal = CI memberi tahu persis contoh
  mana yang bermasalah.
- Pola di luar keluarga matcher saat ini (mis. kelas Konseptual seperti
  chiasmus, atau sisipan seperti tmesis/parenthesis) tetap diterima — CI
  menandainya *jalur manual* untuk review maintainer.

## Alur kerja

1. Buka issue dengan template **"Geometrize a figure"**, klaim satu figur.
2. Fork → branch → edit **satu file** `data/figures/<nama>.json`.
3. Jalankan lokal: `cargo run -p figeometrica-rhetorica --bin validate`
4. Push dan buka PR. CI memverifikasi otomatis.

## Lisensi & atribusi

- Kontribusi dilisensikan **MIT** sejak dibuka PR-nya (inbound = outbound).
- Nama Anda tersimpan di field `atribusi` entri + CONTRIBUTORS.md.
- **Jangan menyalin definisi prosa** dari sumber berhak cipta (Silva
  Rhetoricae dsb.). Struktur taksonomi klasik adalah milik publik; tulisan
  orang lain bukan.

## Co-authorship dataset paper

Kontributor dengan **≥ 10 entri diterima** atau berperan sebagai validator
masuk daftar co-author publikasi dataset. Kriteria final diumumkan sebelum
paper ditulis dan tidak berlaku surut.
