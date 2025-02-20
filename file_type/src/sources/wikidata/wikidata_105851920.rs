use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851920: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_920,
        source_type: SourceType::Wikidata,
        name: "VIA setup configuration file",
        extensions: &["scf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x53, 0x43, 0x46, 0x5D, 0x0D, 0x0A, 0x43, 0x4F, 0x4D, 0x50, 0x41,
                        0x4E, 0x59, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
