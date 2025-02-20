use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111673961: FileType = FileType {
    file_format: &FileFormat {
        id: 111_673_961,
        source_type: SourceType::Wikidata,
        name: "Kingsoft Writer Template",
        extensions: &["wpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
