# The Figeometrica Manifesto · Manifesto Figeometrica

*Figures are geometric — and once you see it, text analysis changes shape.*

*Figur itu geometris — dan begitu terlihat, cara kita menganalisis teks berubah bentuk.*

---

## 1 · The problem

Twenty-four centuries ago, Aristotle systematized persuasion. Roman
rhetoricians catalogued the ornaments of speech; Renaissance schoolbooks
drilled schoolboys on hundreds of them. We inherited roughly 456 named
figures of speech — **the oldest taxonomy of text analysis in existence**,
refined continuously from antiquity through the Renaissance.

And every single one of them is defined in prose.

Prose cannot be executed. Ask a room full of NLP researchers how many texts
in a corpus contain anaphora, and nobody can answer without reading every
text. Ask which passages escalate toward a climax, and you get opinions.
Modern computational methods skipped this layer entirely: Rhetorical
Structure Theory consciously discarded surface form in favor of semantic
relations; stylometry reduced style to function-word statistics; large
language models can *imitate* a style but cannot *audit* one — ask why a
passage feels rhythmic and you receive vibes.

The result: humanity's oldest and most refined theory of how texts are
shaped sits unused by machines.

## 1 · Masalah

Dua puluh empat abad silam, Aristoteles menyistematisasi seni meyakinkan.
Retorikus Romawi mengkatalogkan ornamen tutur; buku sekolah Renaisans
membiasakan para murid dengan ratusan figur. Kita mewarisi kurang lebih 456
figur bahasa yang bernama — **taksonomi analisis teks tertua yang pernah
ada**, terus diasah dari zaman kuno hingga Renaisans.

Dan semuanya didefinisikan dalam prosa.

Prosa tidak bisa dieksekusi. Tanyakan ke ruangan penuh peneliti NLP berapa
banyak teks dalam korpus yang memuat anafora — tak seorang pun bisa menjawab
tanpa membaca satu per satu. Tanyakan bagian mana yang meningkat menuju
klimaks, yang Anda dapat adalah opini. Metode komputasional modern
melewatkan lapisan ini sama sekali: Rhetorical Structure Theory sengaja
membuang bentuk permukaan demi relasi semantik; stilometri mereduksi gaya
menjadi statistik kata fungsi; model bahasa besar bisa *meniru* gaya tetapi
tidak bisa *mengauditnya* — tanyakan mengapa sebuah paragraf terasa
berirama, yang turun adalah kesan-kesanan.

Akibatnya: teori tertua dan tersaring terbaik tentang bagaimana teks
dibentuk duduk menganggur, tak tersentuh mesin.

---

## 2 · The insight

Read the definitions closely and they leak algorithms.

Anaphora: *"repetition of the same word at the beginning of successive
clauses."* That is not prose wearing a definition's clothes — that is an
operation: insert the same token at the **initial anchor** of consecutive
units, repeated at least twice. Antimetabole: invert a phrase — permutation.
Tmesis: cut a word open and insert another inside it — addition at the
grapheme grain. Chiasmus: reverse two conceptual roles across a turn —
permutation over meanings instead of words.

The Romans already knew this. Their four *operae* — **adjectio** (addition),
**detractio** (deletion), **immutatio** (substitution), **transmutatio**
(permutation) — plus repetition are the complete operator set. Every figure
in the catalog is a parameterization of these operations: an anchor point, a
grain, a repeat count, sometimes a slot template.

So we state the thesis plainly:

> A figure definition is an uncompiled algorithm. And a definition that
> *cannot* be written as such an operation is not a "non-geometric figure" —
> it is a badly written definition.

This makes the entire 456-figure catalog falsifiable, for the first time in
its long history.

## 2 · Wawasan

Baca definisinya dengan teliti, dan definisi itu membocorkan algoritma.

Anafora: *"pengulangan kata yang sama di awal klausa-klausa berturutan."*
Itu bukan prosa yang menyamar jadi definisi — itu operasi: sisipkan token
yang sama pada **jangkar awal** unit-unit berturutan, diulang minimal dua
kali. Antimetabole: balikkan sebuah frasa — permutasi. Tmesis: belah sebuah
kata dan sisipkan kata lain di dalamnya — aditio pada satuan grafem.
Kiasmus: balikkan dua peran konseptual lintas giliran — permutasi atas makna,
bukan kata.

Orang Romawi sudah tahu. Empat *operae* mereka — **adjectio** (penambahan),
**detractio** (penghapusan), **immutatio** (penggantian), **transmutatio**
(permutasi) — ditambah repetisi, adalah set operator yang lengkap. Setiap
figur dalam katalog adalah parameterisasi dari operasi-operasi itu: titik
jangkar, satuan, jumlah ulangan, kadang sebuah templat slot.

Maka kami nyatakan tesisnya apa adanya:

> Definisi figur adalah algoritma yang belum dikompilasi. Dan definisi yang
> *tidak bisa* ditulis sebagai operasi semacam itu bukan "figur non-geometris"
> — melainkan definisi yang ditulis buruk.

Untuk pertama kalinya dalam sejarah panjangnya, seluruh katalog 456 figur
itu menjadi dapat difalsifikasi.

---

## 3 · The move

Compile them.

Every definition is rewritten into canonical form:

```
figure = OPERATION × ANCHOR × GRAIN × REPETITION
         (adjectio | detractio | immutatio | transmutatio | repetitio)
         × (initial | final | insertion | whole-unit | cross-unit)
         × (grapheme | word | phrase | unit | discourse)
```

`tmesis` ("abso-bloody-lutely") becomes:

```json
{ "jangkar": "Sisipan", "kelas": "Leksikal", "satuan": "kata",
  "operasi": "adjectio", "minim_ulangan": 1 }
```

Once compiled, everything changes:

- **Detection is deterministic.** The matcher never calls a model. Given a
  text, it either finds the pattern or does not — with byte-exact evidence
  spans.
- **The catalog becomes queryable.** "Which figures close a discourse?" →
  filter by final anchor. "What can escalate?" → gradatio. Before analyzing
  any document.
- **Contributions are machine-checked.** Every entry ships with positive and
  negative example sentences; CI runs the matcher against them. A
  contributor cannot submit a spec that contradicts their own examples.
- **Negative evidence becomes real.** "No chiasmus in this paragraph" stops
  being an impression and becomes a checkable claim.

## 3 · Langkahnya

Kompilasilah.

Setiap definisi ditulis ulang ke dalam bentuk kanonik:

```
figur = OPERASI × JANGKAR × SATUAN × PENGULANGAN
        (adjectio | detractio | immutatio | transmutatio | repetitio)
        × (Awal | Akhir | Sisipan | UnitUtuh | AntarUnit)
        × (grafem | kata | frasa | unit | wacana)
```

`tmesis` ("abso-bloody-lutely") menjadi:

```json
{ "jangkar": "Sisipan", "kelas": "Leksikal", "satuan": "kata",
  "operasi": "adjectio", "minim_ulangan": 1 }
```

Begitu terkompilasi, segalanya berubah:

- **Deteksi bersifat deterministik.** Matcher tidak pernah memanggil model.
  Diberi teks, ia menemukan polanya atau tidak — dengan rentang bukti
  presisi-byte.
- **Katalog bisa di-query.** "Figur apa yang menutup sebuah pidato?" →
  saring jangkar Akhir. "Apa yang bisa meningkat menuju puncak?" → gradatio.
  Semua itu sebelum dokumen mana pun dianalisis.
- **Kontribusi dicek mesin.** Setiap entri membawa contoh kalimat positif
  dan negatif; CI menjalankan matcher atas contoh itu. Kontributor tidak
  mungkin mengajukan spesifikasi yang bertentangan dengan contohnya sendiri.
- **Bukti negatif menjadi nyata.** "Tidak ada kiasmus di paragraf ini"
  berhenti menjadi kesan dan menjadi klaim yang bisa diperiksa.

---

## 4 · Why it matters

**For NLP and computational humanities:** this is the missing bridge between
classical stylistics and computation. Retrieval by rhetorical function —
"find texts that build momentum," "find passages that concede before
refuting" — instead of retrieval by keywords. Style analysis with provenance
instead of vibes.

**For writers and teachers:** figures stop being trivia to memorize and
become moves to see, name, and practice. A student's speech can be checked:
does it open with parallel structure? Does it escalate? Where does it close?
Style becomes teachable because it becomes visible.

**For AI systems:** hybrid pipelines where geometry is the deterministic
evidence layer and language models do what they are good at — interpretation
— on top of evidence they cannot fake. Every finding auditable down to its
byte offsets.

**For the humanities at large:** a demonstration that theories become
cumulative and testable when compiled. Not by reducing them to numbers, but
by taking their structural claims seriously enough to execute them.

## 4 · Kenapa ini penting

**Bagi NLP dan humaniora komputasional:** inilah jembatan yang hilang antara
stilistik klasik dan komputasi. Penelusuran berdasarkan fungsi retoris —
"temukan teks yang membangun momentum", "temukan bagian yang mengalah sebelum
membantah" — alih-alih penelusuran berbasis kata kunci. Analisis gaya dengan
provenance, bukan kesan-kesanan.

**Bagi penulis dan pengajar:** figur berhenti menjadi hafalan dan menjadi
gerakan yang bisa dilihat, dinamai, dilatih. Pidato murid bisa diperiksa:
apakah dibuka dengan struktur paralel? Apakah meningkat ke puncak? Di mana
ditutup? Gaya menjadi bisa diajarkan karena menjadi terlihat.

**Bagi sistem AI:** pipeline hibrida tempat geometri menjadi lapisan bukti
yang deterministik, dan model bahasa melakukan apa yang memang ia kuasai —
interpretasi — di atas bukti yang tidak bisa ia palsukan. Setiap temuan
dapat diaudit sampai ke offset byte-nya.

**Bagi humaniora secara luas:** sebuah demonstrasi bahwa teori menjadi
kumulatif dan teruji bila dikompilasi. Bukan dengan merekayasanya menjadi
angka, melainkan dengan mengambil klaim strukturalnya cukup serius untuk
dieksekusi.

---

## 5 · The bigger frame

Rhetoric is the pilot, not the boundary.

The framework — theory base as versioned data, canonical compilation format,
deterministic engines, machine-validated crowd contributions — applies to
any humanistic theory whose claims have structure. Fallacies come next:
Aristotle's *apparent enthymemes*, arguments that look valid and are not,
waiting for the same treatment. Then prosody, argument schemes, narrative
moves.

The machine does not replace the rhetorician. It gives their oldest
observations executable bodies — so that what was discovered by hand over
twenty-four centuries can finally be verified at scale.

## 5 · Bingkai besar

Retorika adalah pilotnya, bukan batasnya.

Kerangkanya — basis teori sebagai data berversi, format kompilasi kanonik,
mesin deterministik, kontribusi massal yang divalidasi mesin — berlaku untuk
teori humaniora mana pun yang klaimnya memiliki struktur. Fallaciae datang
berikutnya: *apparent enthymeme* Aristoteles, argumen yang tampak sah namun
tidak, menunggu perlakuan yang sama. Lalu prosodi, skema argumen, gerakan
naratif.

Mesin tidak menggantikan retorikus. Mesin memberi pengamatan-pengamatan
tertua mereka tubuh yang tereksekusi — agar apa yang ditemukan secara manual
selama dua puluh empat abad akhirnya bisa diverifikasi secara skala.

---

*447 figures await. Pick one, compile it, let the machine check your work.*

*447 figur menunggu. Pilih satu, kompilasi, biarkan mesin memeriksa
pekerjaanmu.*
