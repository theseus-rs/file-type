use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119163950: FileType = FileType {
    file_format: &FileFormat {
        id: 119_163_950,
        source_type: SourceType::Wikidata,
        name: "Xstart Settings File",
        extensions: &["xs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
