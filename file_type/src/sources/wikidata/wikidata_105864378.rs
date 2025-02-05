use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864378: FileFormat = FileFormat {
    id: 105_864_378,
    source_type: SourceType::Wikidata,
    name: "PVM2 Volume format",
    extensions: &["pvm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x56, 0x4D, 0x32, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
