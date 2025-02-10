use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130404774: FileType = FileType {
    file_format: &FileFormat {
        id: 130_404_774,
        source_type: SourceType::Wikidata,
        name: "OpenEdge ABL source code file",
        extensions: &["p"],
        media_types: &["application/x-openedge", "text/x-openedge"],
        signatures: &[],
        related_formats: &[],
    },
};
