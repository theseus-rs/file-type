use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850290: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_290,
        source_type: SourceType::Wikidata,
        name: "Cinespace LUT (3D)",
        extensions: &["csp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x53, 0x50, 0x4C, 0x55, 0x54, 0x56, 0x31, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
