use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_8043144: FileFormat = FileFormat {
    id: 8_043_144,
    source_type: SourceType::Wikidata,
    name: "Xar",
    extensions: &["xar"],
    media_types: &["application/vnd.xara"],
    internal_signatures: &[],
    related_formats: &[],
};
