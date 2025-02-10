use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856148: FileFormat = FileFormat {
    id: 105_856_148,
    source_type: SourceType::Wikidata,
    name: "Disk Masher System compressed disk image",
    extensions: &["dms", "fms"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4D, 0x53, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
