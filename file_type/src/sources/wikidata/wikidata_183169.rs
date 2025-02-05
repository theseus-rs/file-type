use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_183169: FileFormat = FileFormat {
    id: 183_169,
    source_type: SourceType::Wikidata,
    name: "Jakarta Server Pages",
    extensions: &["jsp"],
    media_types: &["application/jsp"],
    signatures: &[],
    related_formats: &[],
};
