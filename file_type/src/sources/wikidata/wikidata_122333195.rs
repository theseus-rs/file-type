use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122333195: FileFormat = FileFormat {
    id: 122_333_195,
    source_type: SourceType::Wikidata,
    name: "HCK image file",
    extensions: &["hck"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
