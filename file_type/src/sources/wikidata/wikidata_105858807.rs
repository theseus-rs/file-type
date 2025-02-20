use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858807: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_807,
        source_type: SourceType::Wikidata,
        name: "Ipaint bitmap",
        extensions: &["ip"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x42, 0x52, 0x55, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
