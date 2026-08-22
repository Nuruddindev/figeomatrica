# Knowledge v1 — keadaan awal

**Dibuat**: 2026-08-23 · **Sumber**: snapshot tabel knowledge SARVA vault
pasca implementasi CONTRACT.md v1 (Fase 1–5).

## Isi

| Vocabulary | Jumlah | Catatan |
|---|---|---|
| domains | 4 | textual, conceptual, entity, argumentative |
| units | 12 | grapheme…concept (termasuk unit entitas untuk figur personifikasi) |
| scopes | 5 | phonological-form, orthographic-form, token-stream, representation, discourse |
| anchors | 10 | posisi segmen (initial/final/medial), insertion-point, whole-unit, cross-boundary, + anchor entitas (non-person, person, non-human, character) |
| payloads | 7 | segment, letter, syllable, person, human-attribute, characterological-attribute, preemptive-response |
| loci | 9 | initial, medial, terminal, response, distributed, clustered, every, cross_unit, alternating |
| bindings | 9 | semuanya status `valid`; kombinasi lain = UNKNOWN (legal tapi belum diuji) |

## Mengapa keadaan ini jadi v1

Ini titik ketika geometrisasi berpindah dari vault privat SARVA ke ledger
publik figeometrica. Angka-angka di atas bukan desain teoretis — semuanya
lahir dari 455 definisi riil yang diekstraksi/migrasi/disidang:

- anchor entitas (`non-person`, dll.) lahir dari prosopopoeia;
- `preemptive-response` lahir dari procatalepsis;
- locus `alternating` lahir dari abecedarian;
- 9 bindings valid = kombinasi yang benar-benar dipakai figur bersignature.

## Yang TIDAK ada di v1 (dan itu disengaja)

Tidak ada slot "buat jaga-jaga". Setiap slot di sini punya figur pemesan.
Vocabulary kosong yang menggoda (mis. scope generik seperti `other`)
sengaja ditolak sesuai prinsip *scope bukan tempat sampah* (CONTRACT §4).
