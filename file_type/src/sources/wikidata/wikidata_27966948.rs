use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966948: FileFormat = FileFormat {
    id: 27_966_948,
    source_type: SourceType::Wikidata,
    name: "SPC",
    extensions: &["rsn", "spc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
