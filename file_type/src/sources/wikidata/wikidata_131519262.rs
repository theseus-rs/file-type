use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131519262: FileFormat = FileFormat {
    id: 131_519_262,
    source_type: SourceType::Wikidata,
    name: "Stimulate Signal Parameters",
    extensions: &["spr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
