use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27996219: FileType = FileType {
    file_format: &FileFormat {
        id: 27_996_219,
        source_type: SourceType::Wikidata,
        name: "Eudora Mailbox Table of Contents",
        extensions: &["toc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
