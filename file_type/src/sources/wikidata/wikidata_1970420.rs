use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1970420: FileFormat = FileFormat {
    id: 1_970_420,
    source_type: SourceType::Wikidata,
    name: "Simple file verification",
    extensions: &["sfv"],
    media_types: &["text/x-sfv"],
    signatures: &[],
    related_formats: &[],
};
