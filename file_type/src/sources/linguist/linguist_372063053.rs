use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_372063053: FileType = FileType {
    file_format: &FileFormat {
        id: 372_063_053,
        source_type: SourceType::Linguist,
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
        signatures: &[],
        related_formats: &[],
    },
};
