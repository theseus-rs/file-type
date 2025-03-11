use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762726: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_726,
        source_type: SourceType::Wikidata,
        name: "XACT Project",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x69, 0x67, 0x6E, 0x61, 0x74, 0x75, 0x72, 0x65, 0x20, 0x3D, 0x20,
                        0x58, 0x41, 0x43, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
