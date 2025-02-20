use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131850479: FileType = FileType {
    file_format: &FileFormat {
        id: 131_850_479,
        source_type: SourceType::Wikidata,
        name: "OpenVDB file format",
        extensions: &["vdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
