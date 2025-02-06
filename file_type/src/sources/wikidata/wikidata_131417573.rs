use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131417573: FileFormat = FileFormat {
    id: 131_417_573,
    source_type: SourceType::Wikidata,
    name: "FRACT file",
    extensions: &["FRACT"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
