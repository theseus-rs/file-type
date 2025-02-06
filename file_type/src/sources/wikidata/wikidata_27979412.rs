use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979412: FileFormat = FileFormat {
    id: 27_979_412,
    source_type: SourceType::Wikidata,
    name: "RIPscrip",
    extensions: &["rip"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x7C])],
            },
        }],
    }],
    related_formats: &[],
};
