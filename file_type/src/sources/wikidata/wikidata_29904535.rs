use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29904535: FileFormat = FileFormat {
    id: 29_904_535,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System program",
    extensions: &["sas"],
    media_types: &[
        "application/octet-stream",
        "application/x-sas",
        "text/sas",
        "text/x-sas",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
