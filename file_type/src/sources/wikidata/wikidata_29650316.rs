use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650316: FileFormat = FileFormat {
    id: 29_650_316,
    source_type: SourceType::Wikidata,
    name: "Packed Font File Format",
    extensions: &["pk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
