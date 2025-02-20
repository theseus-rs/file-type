use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111190518: FileType = FileType {
    file_format: &FileFormat {
        id: 111_190_518,
        source_type: SourceType::Wikidata,
        name: "Visual Tool Markup Language File",
        extensions: &["vtml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
