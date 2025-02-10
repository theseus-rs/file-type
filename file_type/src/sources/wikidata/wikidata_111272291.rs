use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111272291: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_291,
        source_type: SourceType::Wikidata,
        name: "Ensoniq SQ1/SQ2/KS32 disk image",
        extensions: &["edq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
