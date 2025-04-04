use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852513: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_513,
        source_type: SourceType::Wikidata,
        name: "SupervisionCam Camera Settings",
        extensions: &["svc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x47, 0x65, 0x72, 0x6E, 0x65, 0x72, 0x61, 0x6C, 0x5D, 0x0D, 0x0A,
                        0x41, 0x75, 0x74, 0x6F, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74, 0x20, 0x3D,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
