use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851404: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_404,
        source_type: SourceType::Wikidata,
        name: "RamTracker module",
        extensions: &["trk"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x52, 0x4B, 0x30, 0x31, 0x2F, 0x54, 0x56, 0x2E, 0x45, 0x53, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
