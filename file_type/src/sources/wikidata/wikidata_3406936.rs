use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3406936: FileType = FileType {
    file_format: &FileFormat {
        id: 3_406_936,
        source_type: SourceType::Wikidata,
        name: "Program database",
        extensions: &["pdb"],
        media_types: &["application/x-ms-pdb"],
        signatures: &[],
        related_formats: &[],
    },
};
