use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849611: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_611,
        source_type: SourceType::Wikidata,
        name: "CADe_SIMU project",
        extensions: &["cad"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x41, 0x44, 0x65, 0x5F, 0x53, 0x49, 0x4D, 0x55, 0x2A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
