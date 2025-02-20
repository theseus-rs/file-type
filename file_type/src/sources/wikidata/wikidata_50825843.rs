use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50825843: FileType = FileType {
    file_format: &FileFormat {
        id: 50_825_843,
        source_type: SourceType::Wikidata,
        name: "AVCHD Index File",
        extensions: &["bdm", "bdmv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
