use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850902: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_902,
        source_type: SourceType::Wikidata,
        name: "KeyKit Page",
        extensions: &["kp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x61, 0x67, 0x65, 0x6F, 0x62, 0x6A, 0x3D, 0x24, 0x31, 0x0A, 0x50,
                        0x61, 0x67, 0x65, 0x6E, 0x6D, 0x3D, 0x22, 0x50, 0x61, 0x67, 0x65, 0x20,
                        0x31, 0x22, 0x0A, 0x50, 0x61, 0x67, 0x65, 0x73, 0x7A, 0x3D, 0x5B, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
