use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138573369: FileType = FileType {
    file_format: &FileFormat {
        id: 138_573_369,
        source_type: SourceType::Wikidata,
        name: "Ipe file format",
        extensions: &["ipe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
