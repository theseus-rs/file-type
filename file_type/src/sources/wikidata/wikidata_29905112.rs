use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29905112: FileFormat = FileFormat {
    id: 29_905_112,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System item store file",
    extensions: &["sas7bitm", "sr7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
