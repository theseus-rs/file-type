use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124970136: FileFormat = FileFormat {
    id: 124_970_136,
    source_type: SourceType::Wikidata,
    name: "MIX status file",
    extensions: &["mixstatus"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
