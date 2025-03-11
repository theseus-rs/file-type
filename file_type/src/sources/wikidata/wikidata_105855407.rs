use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855407: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_407,
        source_type: SourceType::Wikidata,
        name: "Fibonacci crunched data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x69, 0x42, 0x6F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
