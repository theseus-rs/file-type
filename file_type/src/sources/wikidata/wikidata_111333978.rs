use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111333978: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_978,
        source_type: SourceType::Wikidata,
        name: "Rockwell 3-bit ADPCM data",
        extensions: &["rockwell-3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
