use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859794: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_794,
        source_type: SourceType::Wikidata,
        name: "VisualBasic Project (Control)",
        extensions: &["vbp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x79, 0x70, 0x65, 0x3D, 0x43, 0x6F, 0x6E, 0x74, 0x72, 0x6F, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
