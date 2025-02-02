use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122333195: FileFormat = FileFormat {
    id: 122_333_195,
    source_type: SourceType::Wikidata,
    name: "HCK image file",
    extensions: &["hck"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
