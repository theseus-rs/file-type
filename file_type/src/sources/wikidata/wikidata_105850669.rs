use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850669: FileFormat = FileFormat {
    id: 105_850_669,
    source_type: SourceType::Wikidata,
    name: "Keyman Virtual Keyboard",
    extensions: &["kvk"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x56, 0x4B, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
