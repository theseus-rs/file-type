use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858526: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_526,
        source_type: SourceType::Wikidata,
        name: "Interpaint bitmap",
        extensions: &["iph", "ipt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x40])],
                },
            }],
        }],
        related_formats: &[],
    },
};
