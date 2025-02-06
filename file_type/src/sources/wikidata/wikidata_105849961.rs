use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849961: FileFormat = FileFormat {
    id: 105_849_961,
    source_type: SourceType::Wikidata,
    name: "ChiWriter document (v3.x or older)",
    extensions: &["chi"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5C, 0x31, 0x63, 0x77])],
            },
        }],
    }],
    related_formats: &[],
};
