use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_622529198: FileFormat = FileFormat {
    id: 622_529_198,
    source_type: SourceType::Linguist,
    name: "Faust",
    extensions: &["dsp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
