use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206883: FileFormat = FileFormat {
    id: 28_206_883,
    source_type: SourceType::Wikidata,
    name: "Planetary Data System",
    extensions: &["img", "imq", "lbl", "pds"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
