use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28756261: FileFormat = FileFormat {
    id: 28_756_261,
    source_type: SourceType::Wikidata,
    name: "FIG",
    extensions: &["fig"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
