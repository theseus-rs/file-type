use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852032: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_032,
        source_type: SourceType::Wikidata,
        name: "SerialBox serials numbers package",
        extensions: &["sb2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6C, 0x42, 0x6F, 0x78, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
