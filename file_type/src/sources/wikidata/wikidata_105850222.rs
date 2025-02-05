use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850222: FileFormat = FileFormat {
    id: 105_850_222,
    source_type: SourceType::Wikidata,
    name: "3ds UI colors",
    extensions: &["clr"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
