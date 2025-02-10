use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_281: FileType = FileType {
    file_format: &FileFormat {
        id: 281,
        source_type: SourceType::Linguist,
        name: "Pascal",
        extensions: &["dfm", "dpr", "inc", "lpr", "pas", "pascal", "pp"],
        media_types: &["text/x-pascal"],
        signatures: &[],
        related_formats: &[],
    },
};
