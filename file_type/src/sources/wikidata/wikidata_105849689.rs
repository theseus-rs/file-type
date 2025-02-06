use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849689: FileFormat = FileFormat {
    id: 105_849_689,
    source_type: SourceType::Wikidata,
    name: "PlayStation RSD Coordinates (gen)",
    extensions: &["cod"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x43, 0x4F, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
