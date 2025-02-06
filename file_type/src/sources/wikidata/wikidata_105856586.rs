use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856586: FileFormat = FileFormat {
    id: 105_856_586,
    source_type: SourceType::Wikidata,
    name: "Wordup Graphics Toolkit Font",
    extensions: &["wfn"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x47, 0x54, 0x20, 0x46, 0x6F, 0x6E, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
