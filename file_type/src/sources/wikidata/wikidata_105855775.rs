use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855775: FileFormat = FileFormat {
    id: 105_855_775,
    source_type: SourceType::Wikidata,
    name: "DNL eBook / eCatalog / eCard / eBrochure",
    extensions: &["dnl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xA5, 0x9D, 0x7A, 0x18])],
            },
        }],
    }],
    related_formats: &[],
};
