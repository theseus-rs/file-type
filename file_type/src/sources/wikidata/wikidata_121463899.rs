use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121463899: FileType = FileType {
    file_format: &FileFormat {
        id: 121_463_899,
        source_type: SourceType::Wikidata,
        name: "Adobe Lightroom Database",
        extensions: &["lrdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
