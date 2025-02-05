use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904449: FileFormat = FileFormat {
    id: 29_904_449,
    source_type: SourceType::Wikidata,
    name: "Psion Series 3 Word",
    extensions: &["wrd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x49, 0x4F, 0x4E, 0x57, 0x50, 0x44, 0x41, 0x54, 0x41, 0x46, 0x49,
                    0x4C, 0x45, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
