use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850755: FileFormat = FileFormat {
    id: 105_850_755,
    source_type: SourceType::Wikidata,
    name: "IBM i (Client) Access Keyboard Map",
    extensions: &["kmp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x72, 0x6F, 0x66, 0x69, 0x6C, 0x65, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
