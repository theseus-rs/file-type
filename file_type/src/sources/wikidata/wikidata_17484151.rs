use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17484151: FileFormat = FileFormat {
    id: 17_484_151,
    source_type: SourceType::Wikidata,
    name: "Scribus Document",
    extensions: &["scd", "scd.gz", "sla", "sla.gz", "slaz"],
    media_types: &["application/vnd.scribus"],
    signatures: &[],
    related_formats: &[],
};
