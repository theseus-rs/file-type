use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856972: FileFormat = FileFormat {
    id: 105_856_972,
    puid: "wikidata/105856972",
    name: "StarWriter for MS-DOS Graphics Printer driver",
    extensions: &["gpm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x20, 0x59, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
