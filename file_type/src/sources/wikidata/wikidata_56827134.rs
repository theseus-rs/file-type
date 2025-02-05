use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_56827134: FileFormat = FileFormat {
    id: 56_827_134,
    source_type: SourceType::Wikidata,
    name: "PicoTech Picologger PLW",
    extensions: &["plw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
