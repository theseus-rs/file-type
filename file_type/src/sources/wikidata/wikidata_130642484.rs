use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130642484: FileFormat = FileFormat {
    id: 130_642_484,
    source_type: SourceType::Wikidata,
    name: "Roboconf instances file",
    extensions: &["instances"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
