use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134286536: FileType = FileType {
    file_format: &FileFormat {
        id: 134_286_536,
        source_type: SourceType::Wikidata,
        name: "Clipper index file",
        extensions: &["nxt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
