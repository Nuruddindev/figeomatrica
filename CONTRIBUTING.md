# Berkontribusi

Aturan mainnya satu kalimat: **klaim harus berbukti.** Semua yang ada di
repo ini bisa diperiksa mesin — termasuk klaim Anda. Gerbangnya jalan di
CI setiap PR, dan versi lokalnya sama persis.

## Menambah / memperbaiki satu figur = satu file JSON

Buka `data/figures/<nama>.json`. Strukturnya:

```json
{
  "id": 91,
  "name": "apocope",
  "definition": "Cutting off final letter/syllable",
  "geometry":   { "...": "blok geometri warisan (opsional)" },
  "signature": {
    "domain_id": "textual",
    "unit_id": "word",
    "scope_id": null,
    "anchor_id": "final-segment",
    "operation": "detractio",
    "payload_id": null,
    "locus_id": null,
    "result": null,
    "constraints": {}
  },
  "epistemic": {
    "status": "WITNESS_TESTED"
  },
  "examples": {
    "positive": [["Photograph", "photo"]],
    "negative": [["The veterinarian examined the dog.",
                  "The vet examined the dog carefully and completely."]]
  }
}
```

### Aturan blok `signature` (CONTRACT.md §2, §12)

- Semua slot **wajib** berasal dari manifest knowledge versi kanon:
  `data/knowledge/vN/manifest.json` — N tertinggi adalah kanon.
- Slot yang tidak relevan: `null`, bukan nilai karangan.
- `scope` bukan tempat sampah: kalau tak yakin, biarkan `null`.

### Aturan blok `epistemic` (tangga status)

| Status Anda klaim | Syarat yang dicek CI |
|---|---|
| `EXTRACTED` | signature ada & slot valid |
| `STRUCTURALLY_VALID` | + bindings bukan INVALID |
| `WITNESS_TESTED` | + protokol witness lulus (CI menjalankan ulang) |
| `USER_ACCEPTED` / `CANONICAL` | + merge oleh maintainer |

Mengklaim status tanpa bukti = PR gagal CI dengan pesan yang menjelaskan
kenapa. Itu fitur, bukan bug: *NO SILENT PROMOTION*.

### Definisi prosa

Tulis dengan kata-kata sendiri. Definisi yang disalin dari sumber
berhak cipta tidak diterima.

## Slot vocabulary yang dibutuhkan belum ada?

Jangan paksa slot lain. Ajukan **versi knowledge baru**:

1. `cp -r data/knowledge/v2 data/knowledge/v3` (atau N tertinggi saat ini)
2. Tambah slot/binding di `v3/manifest.json`
3. Tulis `v3/README.md`: slot apa, figur mana yang membutuhkan, dari
   eksperimen/contoh apa ditemukan
4. Rujuk folder itu di PR Anda

Versi lama tidak pernah diedit — mereka adalah rekam jejak eksperimen.
Detail: [`data/knowledge/README.md`](data/knowledge/README.md).

## Verifikasi lokal sebelum push

```bash
cargo test --workspace
cargo run -q -p figeometrica-rhetorica --bin sidang -- --ci
cargo run -q -p figeometrica-rhetorica --bin validate
```

Kalau ketiganya hijau, CI juga akan hijau.

## Review = Meja Hakim

PR yang lulus CI direview maintainer. Merge adalah tindakan pengesahan:
di situ figur naik ke `USER_ACCEPTED`, dan hanya lewat jalur ini sebuah
figur bisa mencapai `CANONICAL`. Riwayat review tersimpan permanen di
PR — itulah buku besar kami.
