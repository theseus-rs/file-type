use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_18245359: FileFormat = FileFormat {
    id: 18_245_359,
    source_type: SourceType::Wikidata,
    name: "Control Panel Applet",
    extensions: &["cpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
