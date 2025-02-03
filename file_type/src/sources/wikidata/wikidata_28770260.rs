use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28770260: FileFormat = FileFormat {
    id: 28_770_260,
    source_type: SourceType::Wikidata,
    name: "Hugo",
    extensions: &["hex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
