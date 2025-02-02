use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66759540: FileFormat = FileFormat {
    id: 66_759_540,
    source_type: SourceType::Wikidata,
    name: "Excel 97-2003 Template",
    extensions: &["xlt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
