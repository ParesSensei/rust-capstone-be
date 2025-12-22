CREATE TABLE IF NOT EXISTS kuliner (
    id SERIAL PRIMARY KEY,
    nama_makanan TEXT NOT NULL,
    kategori TEXT NOT NULL,
    alamat TEXT NOT NULL,
    harga INT NOT NULL,
    link_gmaps TEXT NOT NULL,
    link_foto TEXT NOT NULL
);
