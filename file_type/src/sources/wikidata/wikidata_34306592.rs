use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34306592: FileFormat = FileFormat {
    id: 34_306_592,
    source_type: SourceType::Wikidata,
    name: "Scifer archive binary header",
    extensions: &["ba"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
