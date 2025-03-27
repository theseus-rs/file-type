use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852503: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_503,
        source_type: SourceType::Wikidata,
        name: "SAP Thomson disk image",
        extensions: &["sap"],
        media_types: &["application/x-thomson-sap-image"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x53, 0x59, 0x53, 0x54, 0x45, 0x4D, 0x45, 0x20, 0x44, 0x27, 0x41,
                        0x52, 0x43, 0x48, 0x49, 0x56, 0x41, 0x47, 0x45, 0x20, 0x50, 0x55, 0x4B,
                        0x41, 0x4C, 0x4C, 0x20, 0x53, 0x2E, 0x41, 0x2E, 0x50, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
