use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_94984970: FileFormat = FileFormat {
    id: 94_984_970,
    source_type: SourceType::Wikidata,
    name: "IGC",
    extensions: &["igc"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
