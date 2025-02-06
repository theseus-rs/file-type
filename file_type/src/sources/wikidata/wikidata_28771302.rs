use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28771302: FileFormat = FileFormat {
    id: 28_771_302,
    source_type: SourceType::Wikidata,
    name: "Matlab figure",
    extensions: &["fig"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
