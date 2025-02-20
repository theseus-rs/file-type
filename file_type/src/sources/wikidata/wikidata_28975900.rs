use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975900: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_900,
        source_type: SourceType::Wikidata,
        name: "Control surface geometry file",
        extensions: &["csf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
