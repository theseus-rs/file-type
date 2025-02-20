use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860516: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_516,
        source_type: SourceType::Wikidata,
        name: "Rigaku XRD RAS format",
        extensions: &["ras"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x52, 0x41, 0x53, 0x5F, 0x44, 0x41, 0x54, 0x41, 0x5F, 0x53, 0x54,
                        0x41, 0x52, 0x54, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
