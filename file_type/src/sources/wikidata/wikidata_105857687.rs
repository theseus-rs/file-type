use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857687: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_687,
        source_type: SourceType::Wikidata,
        name: "QCOW3 disk image",
        extensions: &["img", "qcow3"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x51, 0x46, 0x49, 0xFB, 0x00, 0x00, 0x00, 0x03,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
