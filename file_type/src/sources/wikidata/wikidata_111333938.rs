use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111333938: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_938,
        source_type: SourceType::Wikidata,
        name: "Rockwell 2-bit ADPCM data",
        extensions: &["rockwell-2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
