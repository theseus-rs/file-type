use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_233: FileType = FileType {
    file_format: &FileFormat {
        id: 233,
        source_type: SourceType::Linguist,
        name: "Modelica",
        extensions: &["mo"],
        media_types: &["text/x-modelica"],
        signatures: &[],
        related_formats: &[],
    },
};
