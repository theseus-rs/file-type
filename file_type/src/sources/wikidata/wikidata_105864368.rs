use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864368: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_368,
        source_type: SourceType::Wikidata,
        name: "MSX SME 3 music",
        extensions: &["ply"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4D, 0x45, 0x20, 0x33, 0x20, 0x6D, 0x75, 0x7A, 0x69, 0x65, 0x6B,
                        0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
