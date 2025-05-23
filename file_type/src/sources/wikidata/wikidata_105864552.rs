use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864552: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_552,
        source_type: SourceType::Wikidata,
        name: "PuTTY Private Key (v2)",
        extensions: &["ppk"],
        media_types: &["application/x-putty-private-key"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x75, 0x54, 0x54, 0x59, 0x2D, 0x55, 0x73, 0x65, 0x72, 0x2D, 0x4B,
                        0x65, 0x79, 0x2D, 0x46, 0x69, 0x6C, 0x65, 0x2D, 0x32, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
