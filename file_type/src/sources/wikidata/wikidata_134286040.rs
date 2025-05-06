use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134286040: FileType = FileType {
    file_format: &FileFormat {
        id: 134_286_040,
        source_type: SourceType::Wikidata,
        name: "Clipper executable file",
        extensions: &["exe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
