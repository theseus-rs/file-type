use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_29167914: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_914,
        source_type: SourceType::Wikidata,
        name: "PCRaster Map",
        extensions: &["csf", "map"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x55, 0x55, 0x20, 0x43, 0x52, 0x4F, 0x53, 0x53, 0x20, 0x53, 0x59,
                        0x53, 0x54, 0x45, 0x4D, 0x20, 0x4D, 0x41, 0x50, 0x20, 0x46, 0x4F, 0x52,
                        0x4D, 0x41, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
