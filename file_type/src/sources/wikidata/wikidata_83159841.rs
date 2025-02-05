use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83159841: FileFormat = FileFormat {
    id: 83_159_841,
    source_type: SourceType::Wikidata,
    name: "CRN",
    extensions: &["crn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
