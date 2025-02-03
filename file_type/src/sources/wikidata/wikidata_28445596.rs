use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28445596: FileFormat = FileFormat {
    id: 28_445_596,
    source_type: SourceType::Wikidata,
    name: "APD",
    extensions: &["apd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
