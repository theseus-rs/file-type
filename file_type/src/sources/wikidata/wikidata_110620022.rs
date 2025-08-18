use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110620022: FileType = FileType {
    file_format: &FileFormat {
        id: 110_620_022,
        source_type: SourceType::Wikidata,
        name: "Adobe Atmosphere world (.aer)",
        extensions: &["aer"],
        media_types: &[
            "http://www.wikidata.org/.well-known/genid/68fffe8f4cc632865d28f1bb3c569daa",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
