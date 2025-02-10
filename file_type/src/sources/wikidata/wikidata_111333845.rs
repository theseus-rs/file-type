use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111333845: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_845,
        source_type: SourceType::Wikidata,
        name: "Rockwell ADPCM format (HotFax/Quicklink)",
        extensions: &["rif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
