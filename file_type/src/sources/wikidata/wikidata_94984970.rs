use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_94984970: FileFormat = FileFormat {
    id: 94_984_970,
    source_type: SourceType::Wikidata,
    name: "IGC",
    extensions: &["igc"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
