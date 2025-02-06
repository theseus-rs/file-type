use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130472499: FileFormat = FileFormat {
    id: 130_472_499,
    source_type: SourceType::Wikidata,
    name: "Pig source code file",
    extensions: &["pig"],
    media_types: &["text/x-pig"],
    signatures: &[],
    related_formats: &[],
};
