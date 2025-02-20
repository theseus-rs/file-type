use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_103: FileType = FileType {
    file_format: &FileFormat {
        id: 103,
        source_type: SourceType::Linguist,
        name: "EmberScript",
        extensions: &["em", "emberscript"],
        media_types: &["text/x-coffeescript"],
        signatures: &[],
        related_formats: &[],
    },
};
