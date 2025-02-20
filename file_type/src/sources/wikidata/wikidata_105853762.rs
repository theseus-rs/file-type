use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853762: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_762,
        source_type: SourceType::Wikidata,
        name: "Sonarc compressed RAW PCM audio",
        extensions: &["snc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x6F, 0x6E, 0x61, 0x72, 0x63, 0x2D, 0x73, 0x71, 0x75, 0x65, 0x65,
                        0x7A, 0x65, 0x64, 0x20, 0x50, 0x43, 0x4D, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
