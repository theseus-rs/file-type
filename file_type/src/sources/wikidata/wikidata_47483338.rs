use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47483338: FileFormat = FileFormat {
    id: 47_483_338,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System data set (Unix)",
    extensions: &["sas7bdat", "sd2", "sd7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
