use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864324: FileFormat = FileFormat {
    id: 105_864_324,
    source_type: SourceType::Wikidata,
    name: "Playstation 3 savegame control data",
    extensions: &["pfd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x50, 0x46, 0x44, 0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
