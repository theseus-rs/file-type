use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111333982: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_982,
        source_type: SourceType::Wikidata,
        name: "Rockwell 4-bit ADPCM data",
        extensions: &["rockwell-4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
