use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
