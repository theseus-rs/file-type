use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111317361: FileFormat = FileFormat {
    id: 111_317_361,
    source_type: SourceType::Wikidata,
    name: "MAUD sample format",
    extensions: &["maud"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
