CREATE TABLE IF NOT EXISTS tempat_nongkrong (
    id SERIAL PRIMARY KEY,
    nama_tempat TEXT NOT NULL,
    kategori TEXT NOT NULL,
    alamat TEXT NOT NULL,
    jam_buka TEXT NOT NULL,
    jam_tutup TEXT NOT NULL,
    harga_rata_rata INT NOT NULL,
    link_gmaps TEXT NOT NULL,
    link_foto TEXT NOT NULL
);
