use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205449: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_449,
        source_type: SourceType::Wikidata,
        name: "Design rule for Camera File system THM",
        extensions: &["thm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
