use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855728: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_728,
        source_type: SourceType::Wikidata,
        name: "Device Tree Source",
        extensions: &["dts"],
        media_types: &["text/plain", "text/x-c"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x64, 0x74, 0x73, 0x2D, 0x76, 0x31, 0x2F, 0x3B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
