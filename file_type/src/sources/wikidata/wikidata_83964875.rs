use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83964875: FileType = FileType {
    file_format: &FileFormat {
        id: 83_964_875,
        source_type: SourceType::Wikidata,
        name: "Microsoft Access Workgroup Information File",
        extensions: &["mdw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
