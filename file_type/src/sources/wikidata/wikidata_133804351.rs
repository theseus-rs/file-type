use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133804351: FileType = FileType {
    file_format: &FileFormat {
        id: 133_804_351,
        source_type: SourceType::Wikidata,
        name: "LabEye image",
        extensions: &["im"],
        media_types: &["image/x-labeye-image"],
        signatures: &[],
        related_formats: &[],
    },
};
