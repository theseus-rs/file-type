use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206349: FileFormat = FileFormat {
    id: 28_206_349,
    source_type: SourceType::Wikidata,
    name: "GEOS Convert",
    extensions: &["cvt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
