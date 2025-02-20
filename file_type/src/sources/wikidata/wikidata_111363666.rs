use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111363666: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_666,
        source_type: SourceType::Wikidata,
        name: "Wusikstation file-pack",
        extensions: &["wusikpack"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
