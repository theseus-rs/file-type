use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_75539922: FileFormat = FileFormat {
    id: 75_539_922,
    source_type: SourceType::Wikidata,
    name: "Ulead Private Data",
    extensions: &["upd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
