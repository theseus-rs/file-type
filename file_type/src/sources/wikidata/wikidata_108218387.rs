use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108218387: FileFormat = FileFormat {
    id: 108_218_387,
    source_type: SourceType::Wikidata,
    name: "Citation File Format",
    extensions: &["cff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
