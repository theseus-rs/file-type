use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854716: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_716,
        source_type: SourceType::Wikidata,
        name: "agora bytecode",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2A, 0x60, 0x0A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
