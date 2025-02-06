use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856972: FileFormat = FileFormat {
    id: 105_856_972,
    source_type: SourceType::Wikidata,
    name: "StarWriter for MS-DOS Graphics Printer driver",
    extensions: &["gpm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
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
