use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855478: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_478,
        source_type: SourceType::Wikidata,
        name: "Beyond Words Composer Font",
        extensions: &["fon"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x42, 0x57, 0x46, 0x4F, 0x4E, 0x54, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
