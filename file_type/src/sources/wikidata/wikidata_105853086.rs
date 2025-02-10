use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853086: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_086,
        source_type: SourceType::Wikidata,
        name: "Software Ideas Modeler Template",
        extensions: &["simt"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x73, 0x69, 0x6D, 0x2D, 0x74, 0x65, 0x6D, 0x70, 0x6C, 0x61, 0x74,
                        0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31,
                        0x2E, 0x30, 0x22, 0x3E, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
