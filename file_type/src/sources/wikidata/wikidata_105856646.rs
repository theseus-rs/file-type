use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856646: FileFormat = FileFormat {
    id: 105_856_646,
    puid: "wikidata/105856646",
    name: "AS-EASY-AS Worksheet (generic)",
    extensions: &["wks"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x04, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
