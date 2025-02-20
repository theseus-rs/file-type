use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_32098356: FileType = FileType {
    file_format: &FileFormat {
        id: 32_098_356,
        source_type: SourceType::Wikidata,
        name: "GENMIDI.OP2 OPL2 sound bank",
        extensions: &["op2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
