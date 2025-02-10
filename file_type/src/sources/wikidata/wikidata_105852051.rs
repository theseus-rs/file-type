use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852051: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_051,
        source_type: SourceType::Wikidata,
        name: "snoop verbose trace",
        extensions: &["snoop"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x54, 0x48, 0x45, 0x52, 0x3A, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
