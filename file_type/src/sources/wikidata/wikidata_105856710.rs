use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856710: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_710,
        source_type: SourceType::Wikidata,
        name: "Unique Development Song/module",
        extensions: &["uds"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x8F, 0x4E, 0x47, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
