use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852592: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_592,
        source_type: SourceType::Wikidata,
        name: "SkyChart 2000 Settings",
        extensions: &["scs"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x53, 0x6B, 0x79, 0x43, 0x68, 0x61, 0x72, 0x74, 0x53, 0x65, 0x74,
                        0x74, 0x69, 0x6E, 0x67, 0x73, 0x46, 0x69, 0x6C, 0x65, 0x3E, 0x0D, 0x0A,
                        0x46, 0x69, 0x6C, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
