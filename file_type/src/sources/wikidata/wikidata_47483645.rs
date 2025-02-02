use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47483645: FileFormat = FileFormat {
    id: 47_483_645,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System Data (Windows)",
    extensions: &["sas7bdat", "sd7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
