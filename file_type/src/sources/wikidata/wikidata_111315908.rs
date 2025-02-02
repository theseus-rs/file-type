use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111315908: FileFormat = FileFormat {
    id: 111_315_908,
    source_type: SourceType::Wikidata,
    name: "INRS-Telecommunications audio file",
    extensions: &["inrs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
