use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_55832374: FileFormat = FileFormat {
    id: 55_832_374,
    source_type: SourceType::Wikidata,
    name: "Event Trace Log file format",
    extensions: &["etl"],
    media_types: &["application/x-ms-etl"],
    internal_signatures: &[],
    related_formats: &[],
};
