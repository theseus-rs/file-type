use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867521: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_521,
        source_type: SourceType::Wikidata,
        name: "NUTS format",
        extensions: &["001", "002", "003", "2d", "fid", "spc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x02, 0x03, 0x04, 0x00, 0x01, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
