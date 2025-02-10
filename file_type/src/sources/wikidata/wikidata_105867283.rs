use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867283: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_283,
        source_type: SourceType::Wikidata,
        name: "NetBeans Profiler Snapshot",
        extensions: &["nps"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6E, 0x42, 0x70, 0x52, 0x6F, 0x46, 0x69, 0x4C, 0x65, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
