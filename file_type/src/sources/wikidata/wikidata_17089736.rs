use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17089736: FileType = FileType {
    file_format: &FileFormat {
        id: 17_089_736,
        source_type: SourceType::Wikidata,
        name: "SDIF",
        extensions: &["sdif"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x44, 0x49, 0x46, 0x00, 0x00, 0x00, 0x08,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
