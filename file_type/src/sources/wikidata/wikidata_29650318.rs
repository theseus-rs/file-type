use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650318: FileFormat = FileFormat {
    id: 29_650_318,
    source_type: SourceType::Wikidata,
    name: "PKPass",
    extensions: &["pkpass"],
    media_types: &["application/vnd.apple.pkpass"],
    internal_signatures: &[],
    related_formats: &[],
};
