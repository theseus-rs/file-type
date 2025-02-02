use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206349: FileFormat = FileFormat {
    id: 28_206_349,
    source_type: SourceType::Wikidata,
    name: "GEOS Convert",
    extensions: &["cvt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
