use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3579577: FileType = FileType {
    file_format: &FileFormat {
        id: 3_579_577,
        source_type: SourceType::Wikidata,
        name: "JTAG Chain Information",
        extensions: &["jci"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x4A, 0x54, 0x41, 0x47, 0x2D, 0x43, 0x68, 0x61, 0x69, 0x6E, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
