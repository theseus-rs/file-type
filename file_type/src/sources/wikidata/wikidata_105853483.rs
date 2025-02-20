use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853483: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_483,
        source_type: SourceType::Wikidata,
        name: "ZX-Paintbrush extended document bitmap",
        extensions: &["zxp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x58, 0x2D, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x62, 0x72, 0x75, 0x73,
                        0x68, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6E, 0x64, 0x65, 0x64, 0x20, 0x69,
                        0x6D, 0x61, 0x67, 0x65, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
