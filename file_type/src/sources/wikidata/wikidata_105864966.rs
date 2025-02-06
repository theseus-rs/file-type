use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864966: FileFormat = FileFormat {
    id: 105_864_966,
    source_type: SourceType::Wikidata,
    name: "Descent Player",
    extensions: &["plr"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x4C, 0x50, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
