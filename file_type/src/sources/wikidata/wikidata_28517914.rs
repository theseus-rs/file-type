use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28517914: FileType = FileType {
    file_format: &FileFormat {
        id: 28_517_914,
        source_type: SourceType::Wikidata,
        name: "Sony SR2",
        extensions: &["sr2"],
        media_types: &["image/x-sony-sr2"],
        signatures: &[],
        related_formats: &[],
    },
};
