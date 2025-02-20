use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857003: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_003,
        source_type: SourceType::Wikidata,
        name: "Symantec Guard Header",
        extensions: &["grd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x47, 0x75, 0x61, 0x72, 0x64, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
                        0x5D, 0x0D, 0x0A, 0x4C, 0x65, 0x67, 0x61, 0x6C, 0x3D, 0x43, 0x6F, 0x70,
                        0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
