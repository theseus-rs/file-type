use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1134089: FileType = FileType {
    file_format: &FileFormat {
        id: 1_134_089,
        source_type: SourceType::Wikidata,
        name: "world file",
        extensions: &[
            "bilw", "blw", "bpw", "btw", "gfw", "jgw", "jpgw", "pgw", "rasterw", "sdw", "tfw",
            "tifw",
        ],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
