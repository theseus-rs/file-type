use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
