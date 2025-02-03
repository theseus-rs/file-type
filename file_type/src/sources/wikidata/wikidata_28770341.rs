use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28770341: FileFormat = FileFormat {
    id: 28_770_341,
    source_type: SourceType::Wikidata,
    name: "M2k",
    extensions: &["m2k"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
