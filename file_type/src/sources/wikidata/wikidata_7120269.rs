use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7120269: FileType = FileType {
    file_format: &FileFormat {
        id: 7_120_269,
        source_type: SourceType::Wikidata,
        name: "PRC",
        extensions: &["prc"],
        media_types: &["application/octet-stream", "application/vnd.palm"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x61, 0x70, 0x70, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
