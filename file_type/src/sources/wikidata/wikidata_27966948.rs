use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966948: FileFormat = FileFormat {
    id: 27_966_948,
    source_type: SourceType::Wikidata,
    name: "SPC",
    extensions: &["rsn", "spc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
