use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851556: FileFormat = FileFormat {
    id: 105_851_556,
    puid: "wikidata/105851556",
    name: "Type Library (Type1)",
    extensions: &["tlb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4C, 0x54, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
