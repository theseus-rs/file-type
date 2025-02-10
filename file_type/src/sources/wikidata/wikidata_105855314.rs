use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855314: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_314,
        source_type: SourceType::Wikidata,
        name: "FASTGEN Model",
        extensions: &["fg4"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x24, 0x43, 0x4F, 0x4D, 0x4D, 0x45, 0x4E, 0x54, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
