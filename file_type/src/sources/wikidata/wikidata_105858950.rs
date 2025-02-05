use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858950: FileFormat = FileFormat {
    id: 105_858_950,
    source_type: SourceType::Wikidata,
    name: "Ludek Maker bitmap",
    extensions: &["ldm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xCC, 0xF5, 0xE4, 0xE5, 0xEB, 0xA0, 0xCD, 0xE1, 0xEB, 0xE5, 0xF2, 0xA0, 0xE4,
                    0xE1, 0xF4, 0xE1, 0xA0, 0xE6, 0xE9, 0xEC, 0xE5,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
