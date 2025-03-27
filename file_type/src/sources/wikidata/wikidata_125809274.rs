use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125809274: FileType = FileType {
    file_format: &FileFormat {
        id: 125_809_274,
        source_type: SourceType::Wikidata,
        name: "Bzip2-Compressed TAR File",
        extensions: &["tbz2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
