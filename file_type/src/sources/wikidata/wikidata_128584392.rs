use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128584392: FileType = FileType {
    file_format: &FileFormat {
        id: 128_584_392,
        source_type: SourceType::Wikidata,
        name: "ABNF file format",
        extensions: &["abnf"],
        media_types: &["text/x-abnf"],
        signatures: &[],
        related_formats: &[],
    },
};
