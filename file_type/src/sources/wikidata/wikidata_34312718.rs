use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34312718: FileType = FileType {
    file_format: &FileFormat {
        id: 34_312_718,
        source_type: SourceType::Wikidata,
        name: "Macromedia Director, compressed Macintosh variant",
        extensions: &["dcr"],
        media_types: &["application/x-director"],
        signatures: &[],
        related_formats: &[],
    },
};
