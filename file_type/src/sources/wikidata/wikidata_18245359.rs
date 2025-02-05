use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_18245359: FileFormat = FileFormat {
    id: 18_245_359,
    source_type: SourceType::Wikidata,
    name: "Control Panel Applet",
    extensions: &["cpl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
