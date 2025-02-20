use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_298: FileType = FileType {
    file_format: &FileFormat {
        id: 298,
        source_type: SourceType::Linguist,
        name: "Public Key",
        extensions: &["asc", "pub"],
        media_types: &["application/pgp"],
        signatures: &[],
        related_formats: &[],
    },
};
