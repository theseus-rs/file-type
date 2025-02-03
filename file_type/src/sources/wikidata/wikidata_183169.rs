use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_183169: FileFormat = FileFormat {
    id: 183_169,
    source_type: SourceType::Wikidata,
    name: "Jakarta Server Pages",
    extensions: &["jsp"],
    media_types: &["application/jsp"],
    internal_signatures: &[],
    related_formats: &[],
};
