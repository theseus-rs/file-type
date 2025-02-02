use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_33515758: FileFormat = FileFormat {
    id: 33_515_758,
    source_type: SourceType::Wikidata,
    name: "LAS 1.4 file format",
    extensions: &["las"],
    media_types: &["application/vnd.las"],
    internal_signatures: &[],
    related_formats: &[],
};
