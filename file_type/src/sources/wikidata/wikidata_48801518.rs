use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48801518: FileType = FileType {
    file_format: &FileFormat {
        id: 48_801_518,
        source_type: SourceType::Wikidata,
        name: "Adobe FrameMaker Document, version 8",
        extensions: &["fm"],
        media_types: &["application/vnd.framemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
