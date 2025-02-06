use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114079055: FileFormat = FileFormat {
    id: 114_079_055,
    source_type: SourceType::Wikidata,
    name: "MacBinary III",
    extensions: &["bin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
