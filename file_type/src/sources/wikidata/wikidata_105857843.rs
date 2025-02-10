use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857843: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_843,
        source_type: SourceType::Wikidata,
        name: "BlackBerry Backup",
        extensions: &["ipd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x6E, 0x74, 0x65, 0x72, 0x40, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20,
                        0x50, 0x61, 0x67, 0x65, 0x72, 0x20, 0x42, 0x61, 0x63, 0x6B, 0x75, 0x70,
                        0x2F, 0x52, 0x65, 0x73, 0x74, 0x6F, 0x72, 0x65, 0x20, 0x46, 0x69, 0x6C,
                        0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
