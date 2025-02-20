use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111418325: FileType = FileType {
    file_format: &FileFormat {
        id: 111_418_325,
        source_type: SourceType::Wikidata,
        name: "Adobe Bridge Workspace File",
        extensions: &["workspace"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
