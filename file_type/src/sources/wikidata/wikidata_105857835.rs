use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857835: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_835,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine spell (v2.0)",
        extensions: &["spl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x50, 0x4C, 0x20, 0x56, 0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
