use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856276: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_276,
        source_type: SourceType::Wikidata,
        name: "DAUB drawing (v2.x)",
        extensions: &["dob"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x44, 0x42, 0x42, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
