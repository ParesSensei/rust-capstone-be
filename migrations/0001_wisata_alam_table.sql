CREATE TABLE IF NOT EXISTS wisata_alam (
    id SERIAL PRIMARY KEY,
    nama_tempat TEXT NOT NULL,
    kategori TEXT NOT NULL,
    alamat TEXT NOT NULL,
    jam_buka TEXT NOT NULL,
    jam_tutup TEXT NOT NULL,
    htm INT NOT NULL,
    link_gmaps TEXT NOT NULL,
    link_foto TEXT NOT NULL
);
