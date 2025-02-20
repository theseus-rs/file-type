use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131994277: FileType = FileType {
    file_format: &FileFormat {
        id: 131_994_277,
        source_type: SourceType::Wikidata,
        name: "Web Archive Transformation",
        extensions: &["wat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
