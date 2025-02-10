use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122229335: FileType = FileType {
    file_format: &FileFormat {
        id: 122_229_335,
        source_type: SourceType::Wikidata,
        name: "WPA-PSK Export Hash",
        extensions: &["wph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
