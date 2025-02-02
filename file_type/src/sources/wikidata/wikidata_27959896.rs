use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959896: FileFormat = FileFormat {
    id: 27_959_896,
    source_type: SourceType::Wikidata,
    name: "Nuendo arrangement",
    extensions: &["npr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
