use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34306592: FileFormat = FileFormat {
    id: 34_306_592,
    source_type: SourceType::Wikidata,
    name: "Scifer archive binary header",
    extensions: &["ba"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
