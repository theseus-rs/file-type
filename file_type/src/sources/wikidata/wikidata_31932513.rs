use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_31932513: FileType = FileType {
    file_format: &FileFormat {
        id: 31_932_513,
        source_type: SourceType::Wikidata,
        name: "Linked Data Patch Format",
        extensions: &["ldp"],
        media_types: &["text/ldpatch"],
        signatures: &[],
        related_formats: &[],
    },
};
