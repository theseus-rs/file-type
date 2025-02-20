use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858498: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_498,
        source_type: SourceType::Wikidata,
        name: "LSS16 SYSLINUX Splash image",
        extensions: &["16", "lss"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3D, 0xF3, 0x13, 0x14])],
                },
            }],
        }],
        related_formats: &[],
    },
};
