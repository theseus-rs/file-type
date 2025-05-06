use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134286810: FileType = FileType {
    file_format: &FileFormat {
        id: 134_286_810,
        source_type: SourceType::Wikidata,
        name: "Clipper label definition file",
        extensions: &["lbl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
