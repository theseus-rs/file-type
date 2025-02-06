use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130642484: FileFormat = FileFormat {
    id: 130_642_484,
    source_type: SourceType::Wikidata,
    name: "Roboconf instances file",
    extensions: &["instances"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
