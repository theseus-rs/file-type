use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851556: FileFormat = FileFormat {
    id: 105_851_556,
    source_type: SourceType::Wikidata,
    name: "Type Library (Type1)",
    extensions: &["tlb"],
    media_types: &[],
    signatures: &[Signature {
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
