use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856125: FileFormat = FileFormat {
    id: 105_856_125,
    source_type: SourceType::Wikidata,
    name: "DeSmuME savestate (gen)",
    extensions: &["ds1", "ds4"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x65, 0x53, 0x6D, 0x75, 0x4D, 0x45, 0x20, 0x53, 0x53, 0x74, 0x61, 0x74,
                    0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
