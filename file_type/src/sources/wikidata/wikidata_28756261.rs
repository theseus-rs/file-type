use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28756261: FileFormat = FileFormat {
    id: 28_756_261,
    source_type: SourceType::Wikidata,
    name: "FIG",
    extensions: &["fig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
