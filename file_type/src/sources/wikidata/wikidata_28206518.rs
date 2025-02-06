use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206518: FileFormat = FileFormat {
    id: 28_206_518,
    source_type: SourceType::Wikidata,
    name: "Lucasfilm picture",
    extensions: &["lff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
