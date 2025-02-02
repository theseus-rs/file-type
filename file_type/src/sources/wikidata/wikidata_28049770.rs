use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28049770: FileFormat = FileFormat {
    id: 28_049_770,
    source_type: SourceType::Wikidata,
    name: "DKBTrace scene description",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
