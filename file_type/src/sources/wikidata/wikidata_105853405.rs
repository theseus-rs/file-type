use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853405: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_405,
        source_type: SourceType::Wikidata,
        name: "Imageworks 3D LUT format",
        extensions: &["spi3d"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x50, 0x49, 0x4C, 0x55, 0x54, 0x20, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
