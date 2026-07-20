use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138348487: FileType = FileType {
    file_format: &FileFormat {
        id: 138_348_487,
        source_type: SourceType::Wikidata,
        name: "Autodesk ReCap RCS Indexed Point Cloud Data file",
        extensions: &["rcs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
