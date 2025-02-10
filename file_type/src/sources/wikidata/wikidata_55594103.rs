use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_55594103: FileType = FileType {
    file_format: &FileFormat {
        id: 55_594_103,
        source_type: SourceType::Wikidata,
        name: "CAChe MolStruct CSF",
        extensions: &["csf"],
        media_types: &["chemical/x-cache-csf"],
        signatures: &[],
        related_formats: &[],
    },
};
