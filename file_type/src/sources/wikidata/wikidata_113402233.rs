use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113402233: FileFormat = FileFormat {
    id: 113_402_233,
    source_type: SourceType::Wikidata,
    name: "ZBrush MatCap",
    extensions: &["zmt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
