use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76515169: FileFormat = FileFormat {
    id: 76_515_169,
    source_type: SourceType::Wikidata,
    name: "Windows Runtime Metadata",
    extensions: &["winmd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
