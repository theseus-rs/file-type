use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206883: FileFormat = FileFormat {
    id: 28_206_883,
    source_type: SourceType::Wikidata,
    name: "Planetary Data System",
    extensions: &["img", "imq", "lbl", "pds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
