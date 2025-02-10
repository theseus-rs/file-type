use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853473: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_473,
        source_type: SourceType::Wikidata,
        name: "EightyOne snapshot",
        extensions: &["z81"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x43, 0x50, 0x55, 0x5D, 0x0D, 0x0A, 0x50, 0x43, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
