use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28731046: FileFormat = FileFormat {
    id: 28_731_046,
    source_type: SourceType::Wikidata,
    name: "APL Transfer File format",
    extensions: &["atf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
