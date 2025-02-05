use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864129: FileFormat = FileFormat {
    id: 105_864_129,
    source_type: SourceType::Wikidata,
    name: "SuperKey Macro",
    extensions: &["mac"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x42, 0x45, 0x47, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
