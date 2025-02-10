use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_59660182: FileType = FileType {
    file_format: &FileFormat {
        id: 59_660_182,
        source_type: SourceType::Wikidata,
        name: "qcow2",
        extensions: &["img", "qcow2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x51, 0x46, 0x49, 0xFB, 0x00, 0x00, 0x00, 0x02,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
