use crate::format::FileFormat;

pub(crate) const LINGUIST_372063053: FileFormat = FileFormat {
    id: 372_063_053,
    puid: "linguist/372063053",
    name: "Checksums",
    extensions: &[
        "crc32",
        "md2",
        "md4",
        "md5",
        "sha1",
        "sha2",
        "sha224",
        "sha256",
        "sha256sum",
        "sha3",
        "sha384",
        "sha512",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
