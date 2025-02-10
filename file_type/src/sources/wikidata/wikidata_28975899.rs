use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975899: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_899,
        source_type: SourceType::Wikidata,
        name: "MultiSurf geometry file",
        extensions: &["ms2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
