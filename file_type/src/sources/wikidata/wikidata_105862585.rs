use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862585: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_585,
        source_type: SourceType::Wikidata,
        name: "Meshwork model (v1.1)",
        extensions: &["mesh"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x65, 0x73, 0x68, 0x09, 0x31, 0x09, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
