use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207114: FileFormat = FileFormat {
    id: 28_207_114,
    source_type: SourceType::Wikidata,
    name: "The New Print Shop Names file",
    extensions: &["pnm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
