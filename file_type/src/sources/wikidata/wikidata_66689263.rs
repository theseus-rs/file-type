use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66689263: FileType = FileType {
    file_format: &FileFormat {
        id: 66_689_263,
        source_type: SourceType::Wikidata,
        name: "Access (SQL Server) detached database",
        extensions: &["mdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
