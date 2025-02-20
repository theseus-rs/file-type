use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_17149857: FileType = FileType {
    file_format: &FileFormat {
        id: 17_149_857,
        source_type: SourceType::Wikidata,
        name: "zone file",
        extensions: &["zone"],
        media_types: &["text/dns"],
        signatures: &[],
        related_formats: &[],
    },
};
