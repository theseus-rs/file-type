use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2816480: FileType = FileType {
    file_format: &FileFormat {
        id: 2_816_480,
        source_type: SourceType::Wikidata,
        name: "3DXML",
        extensions: &["3dxml"],
        media_types: &["application/x-3dxmlplugin"],
        signatures: &[],
        related_formats: &[],
    },
};
