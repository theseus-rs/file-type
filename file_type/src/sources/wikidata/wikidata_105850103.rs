use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850103: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_103,
        source_type: SourceType::Wikidata,
        name: ".c3 file",
        extensions: &["c3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x41, 0x58, 0x46, 0x49, 0x4C, 0x45, 0x20, 0x43, 0x33, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
