use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123436289: FileFormat = FileFormat {
    id: 123_436_289,
    source_type: SourceType::Wikidata,
    name: "DARC-F1 file",
    extensions: &["f1d"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
