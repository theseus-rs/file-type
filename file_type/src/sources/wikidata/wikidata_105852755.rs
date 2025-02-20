use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852755: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_755,
        source_type: SourceType::Wikidata,
        name: "Silo 3D model (ascii)",
        extensions: &["sia"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2D, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
