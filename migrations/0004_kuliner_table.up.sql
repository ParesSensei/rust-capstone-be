create table kuliner (
    id serial primary key,
    nama_tempat text not null,
    kategori text not null,
    alamat text not null,
    jam_buka time not null,
    jam_tutup time not null,
    htm int not null,
    link_gmaps text not null,
    link_foto text not null
);