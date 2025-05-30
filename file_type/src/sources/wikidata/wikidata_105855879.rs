use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855879: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_879,
        source_type: SourceType::Wikidata,
        name: "Case Closed game data archive",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x50, 0x49, 0x53, 0x26, 0x4C, 0x5A, 0x48,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
