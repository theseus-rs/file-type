use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4652973: FileFormat = FileFormat {
    id: 4_652_973,
    source_type: SourceType::Wikidata,
    name: "ANIM",
    extensions: &["anim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
