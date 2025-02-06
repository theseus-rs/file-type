use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856575: FileFormat = FileFormat {
    id: 105_856_575,
    source_type: SourceType::Wikidata,
    name: "Wang Virtual Disk image",
    extensions: &["wvd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x41, 0x4E, 0x47, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
