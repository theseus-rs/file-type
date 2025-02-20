use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854479: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_479,
        source_type: SourceType::Wikidata,
        name: "Sonarc compressed VOC audio",
        extensions: &["vc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x6F, 0x6E, 0x61, 0x72, 0x63, 0x2D, 0x73, 0x71, 0x75, 0x65, 0x65,
                        0x7A, 0x65, 0x64, 0x20, 0x56, 0x4F, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
