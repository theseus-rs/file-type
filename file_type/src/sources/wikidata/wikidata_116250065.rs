use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116250065: FileFormat = FileFormat {
    id: 116_250_065,
    source_type: SourceType::Wikidata,
    name: "Norton System Doctor configuration file",
    extensions: &["nsd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
