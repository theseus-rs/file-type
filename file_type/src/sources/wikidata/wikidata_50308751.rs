use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50308751: FileFormat = FileFormat {
    id: 50_308_751,
    source_type: SourceType::Wikidata,
    name: "MIME Email format",
    extensions: &["eml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
