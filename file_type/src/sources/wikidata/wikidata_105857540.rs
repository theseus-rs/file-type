use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857540: FileFormat = FileFormat {
    id: 105_857_540,
    source_type: SourceType::Wikidata,
    name: "SMSQ/E hard disk image",
    extensions: &["win"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x4C, 0x57, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
