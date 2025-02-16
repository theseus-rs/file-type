use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_132230252: FileType = FileType {
    file_format: &FileFormat {
        id: 132_230_252,
        source_type: SourceType::Wikidata,
        name: "Not eXactly C source code file",
        extensions: &["nxc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
