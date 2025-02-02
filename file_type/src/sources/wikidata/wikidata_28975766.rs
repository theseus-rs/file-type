use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975766: FileFormat = FileFormat {
    id: 28_975_766,
    source_type: SourceType::Wikidata,
    name: "DMO format",
    extensions: &["dmo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
