use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_95985389: FileType = FileType {
    file_format: &FileFormat {
        id: 95_985_389,
        source_type: SourceType::Wikidata,
        name: "Affymetrix CHP file format",
        extensions: &["chp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
