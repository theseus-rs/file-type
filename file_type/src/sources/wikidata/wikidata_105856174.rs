use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856174: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_174,
        source_type: SourceType::Wikidata,
        name: "DVD Cutting Master Format DDVID.DAT (v1.20)",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x44, 0x56, 0x20, 0x31, 0x2E, 0x32, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
