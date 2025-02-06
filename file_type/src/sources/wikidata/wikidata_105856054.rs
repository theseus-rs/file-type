use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856054: FileFormat = FileFormat {
    id: 105_856_054,
    source_type: SourceType::Wikidata,
    name: "Borland DataBase Explorer Information",
    extensions: &["dbi"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x0A, 0x0D, 0x0A, 0x5B, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x5D,
                    0x0D, 0x0A, 0x44, 0x62, 0x58,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
