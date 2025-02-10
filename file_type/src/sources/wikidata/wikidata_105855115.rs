use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855115: FileFormat = FileFormat {
    id: 105_855_115,
    source_type: SourceType::Wikidata,
    name: "Akoma Ntoso document",
    extensions: &["xml"],
    media_types: &["application/akn+xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
