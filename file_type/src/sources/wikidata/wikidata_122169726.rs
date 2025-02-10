use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122169726: FileType = FileType {
    file_format: &FileFormat {
        id: 122_169_726,
        source_type: SourceType::Wikidata,
        name: "crypt() Password Hash",
        extensions: &["crypt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
