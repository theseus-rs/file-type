use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122169903: FileType = FileType {
    file_format: &FileFormat {
        id: 122_169_903,
        source_type: SourceType::Wikidata,
        name: "PPA and PWDUMP Password Hashes",
        extensions: &["hdc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
