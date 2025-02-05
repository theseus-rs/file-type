use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979397: FileFormat = FileFormat {
    id: 27_979_397,
    source_type: SourceType::Wikidata,
    name: "Imagic Film/Picture, low resolution",
    extensions: &["ic1"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4D, 0x44, 0x43, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
