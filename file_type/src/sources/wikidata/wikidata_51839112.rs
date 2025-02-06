use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51839112: FileFormat = FileFormat {
    id: 51_839_112,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Film Roll",
    extensions: &["flm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
