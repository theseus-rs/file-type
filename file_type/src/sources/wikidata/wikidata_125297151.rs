use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125297151: FileFormat = FileFormat {
    id: 125_297_151,
    source_type: SourceType::Wikidata,
    name: "cdb format",
    extensions: &["cdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
