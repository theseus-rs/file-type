use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100243992: FileFormat = FileFormat {
    id: 100_243_992,
    source_type: SourceType::Wikidata,
    name: "Student Writing Center Sign",
    extensions: &["sg", "sgt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
