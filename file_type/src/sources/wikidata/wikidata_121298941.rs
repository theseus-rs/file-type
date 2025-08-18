use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121298941: FileType = FileType {
    file_format: &FileFormat {
        id: 121_298_941,
        source_type: SourceType::Wikidata,
        name: "age",
        extensions: &["age"],
        media_types: &[
            "http://www.wikidata.org/.well-known/genid/dde11df646562c62d3d538844b250354",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
