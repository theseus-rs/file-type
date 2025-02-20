use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34737296: FileType = FileType {
    file_format: &FileFormat {
        id: 34_737_296,
        source_type: SourceType::Wikidata,
        name: "Skencil SK",
        extensions: &["sk"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x23, 0x53, 0x6B, 0x65, 0x74, 0x63, 0x68, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
