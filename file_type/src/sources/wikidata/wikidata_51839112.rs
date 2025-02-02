use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51839112: FileFormat = FileFormat {
    id: 51_839_112,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Film Roll",
    extensions: &["flm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
