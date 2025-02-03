use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125415086: FileFormat = FileFormat {
    id: 125_415_086,
    source_type: SourceType::Wikidata,
    name: "TCM document",
    extensions: &["tcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
