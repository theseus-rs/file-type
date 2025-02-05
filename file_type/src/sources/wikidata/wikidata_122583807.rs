use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122583807: FileFormat = FileFormat {
    id: 122_583_807,
    source_type: SourceType::Wikidata,
    name: "Zetafax Fax Image File (Fine)",
    extensions: &["g3f"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
