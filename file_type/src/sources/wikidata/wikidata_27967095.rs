use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967095: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_095,
        source_type: SourceType::Wikidata,
        name: "Inverse Frequency Sound format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4E, 0x44, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
