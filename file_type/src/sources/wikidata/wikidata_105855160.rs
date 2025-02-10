use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855160: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_160,
        source_type: SourceType::Wikidata,
        name: "Frodo SnapShot",
        extensions: &["fss"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x72, 0x6F, 0x64, 0x6F, 0x53, 0x6E, 0x61, 0x70, 0x73, 0x68, 0x6F,
                        0x74, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
