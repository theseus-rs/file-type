use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207561: FileFormat = FileFormat {
    id: 28_207_561,
    source_type: SourceType::Wikidata,
    name: "Xim",
    extensions: &["xim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
