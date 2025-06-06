use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853123: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_123,
        source_type: SourceType::Wikidata,
        name: "ep32 snapshot",
        extensions: &["e32"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x65, 0x70, 0x33, 0x32, 0x2D, 0x30, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
