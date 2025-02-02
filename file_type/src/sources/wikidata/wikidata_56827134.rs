use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_56827134: FileFormat = FileFormat {
    id: 56_827_134,
    source_type: SourceType::Wikidata,
    name: "PicoTech Picologger PLW",
    extensions: &["plw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
