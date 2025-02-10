use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58959314: FileType = FileType {
    file_format: &FileFormat {
        id: 58_959_314,
        source_type: SourceType::Wikidata,
        name: "Microsoft Office Theme",
        extensions: &["thmx"],
        media_types: &["application/vnd.ms-officetheme"],
        signatures: &[],
        related_formats: &[],
    },
};
