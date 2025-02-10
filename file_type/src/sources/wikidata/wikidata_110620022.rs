use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110620022: FileType = FileType {
    file_format: &FileFormat {
        id: 110_620_022,
        source_type: SourceType::Wikidata,
        name: "Adobe Atmosphere world (.aer)",
        extensions: &["aer"],
        media_types: &[
            "http://www.wikidata.org/.well-known/genid/64e48b81d07f7c6707af81a93ee3a882",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
