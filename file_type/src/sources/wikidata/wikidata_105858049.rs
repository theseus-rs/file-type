use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858049: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_049,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine Area (v9.1)",
        extensions: &["are"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x52, 0x45, 0x41, 0x56, 0x39, 0x2E, 0x31, 0x41, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
