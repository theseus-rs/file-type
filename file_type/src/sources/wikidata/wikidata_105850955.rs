use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850955: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_955,
        source_type: SourceType::Wikidata,
        name: "GNU TeXmacs document",
        extensions: &["tm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x54, 0x65, 0x58, 0x6D, 0x61, 0x63, 0x73, 0x7C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
