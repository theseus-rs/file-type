use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134286127: FileType = FileType {
    file_format: &FileFormat {
        id: 134_286_127,
        source_type: SourceType::Wikidata,
        name: "Clipper data file",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
