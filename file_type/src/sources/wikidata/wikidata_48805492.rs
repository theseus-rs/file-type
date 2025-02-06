use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48805492: FileFormat = FileFormat {
    id: 48_805_492,
    source_type: SourceType::Wikidata,
    name: "ChiWriter Document",
    extensions: &["chi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
