use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600492: FileFormat = FileFormat {
    id: 28_600_492,
    source_type: SourceType::Wikidata,
    name: "Data Resource File",
    extensions: &["drs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
