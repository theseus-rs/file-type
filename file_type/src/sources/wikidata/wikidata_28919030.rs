use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919030: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_030,
        source_type: SourceType::Wikidata,
        name: "AC-3 Compressed Audio (Dolby Digital), Revision A",
        extensions: &["ac3"],
        media_types: &["audio/ac3"],
        signatures: &[],
        related_formats: &[],
    },
};
