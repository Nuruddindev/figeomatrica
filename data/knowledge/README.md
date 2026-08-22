# Knowledge Vocabulary — Versi & Manifest

Vocabulary kontrak (domains, units, scopes, anchors, payloads, loci,
bindings) berevolusi setiap kali eksperimen menemukan fenomena yang tak
muat di slot lama. Supaya perkembangan itu **terekam dan bisa diaudit**,
setiap keadaan vocabulary disimpan sebagai satu folder versi:

```
knowledge/
  v1/manifest.json   ← keadaan awal (snapshot SARVA pasca Fase 1–5)
  v1/README.md          mengapa v1 ada, apa isinya
  v2/...                ← lahir saat ada penemuan baru
```

## Aturan versi

1. **Versi tertinggi = kanon.** Bin `sidang` selalu memvalidasi terhadap
   nomor versi tertinggi. Tidak ada pointer `LATEST` yang berubah-ubah —
   git history cukup.
2. **Versi lama abadi.** Setelah dirilis, isi folder versi tidak boleh
   diedit. Penemuan baru = versi baru, bukan koreksi diam-diam. Inilah
   penerapan *NO SILENT PROMOTION* pada tingkat vocabulary.
3. **Naik versi wajib beralasan.** `vN+1/README.md` menjelaskan slot apa
   yang baru, figur mana yang membutuhkannya, dan dari eksperimen apa
   ditemukan. Slot tanpa cerita tidak masuk.
4. **Slot punya status.** `known` = sudah teruji lintas figur;
   `candidate` = dipakai satu-dua figur, menunggu konfirmasi. CI hanya
   menuntut slot `candidate` punya alasan di manifest.

## Cara naik versi

```bash
cp -r data/knowledge/v1 data/knowledge/v2
# edit v2/manifest.json: tambah/ubah slot + status
# tulis v2/README.md: cerita penemuannya
git commit -m "knowledge v2: <slot baru> untuk <figur/fenomena>"
```
