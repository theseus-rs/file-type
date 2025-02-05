use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3564764: FileFormat = FileFormat {
    id: 3_564_764,
    source_type: SourceType::Wikidata,
    name: "Word Perfect Graphic 1.0",
    extensions: &["wpg"],
    media_types: &["application/x-wpg"],
    signatures: &[],
    related_formats: &[],
};
