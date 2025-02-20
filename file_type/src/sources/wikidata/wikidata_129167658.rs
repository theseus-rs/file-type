use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129167658: FileType = FileType {
    file_format: &FileFormat {
        id: 129_167_658,
        source_type: SourceType::Wikidata,
        name: "Ezhil file format",
        extensions: &["n"],
        media_types: &["text/x-ezhil"],
        signatures: &[],
        related_formats: &[],
    },
};
