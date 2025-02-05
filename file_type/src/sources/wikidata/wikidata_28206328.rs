use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206328: FileFormat = FileFormat {
    id: 28_206_328,
    source_type: SourceType::Wikidata,
    name: "Img Software Set Red Component",
    extensions: &["r"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
