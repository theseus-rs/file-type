use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854918: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_918,
        source_type: SourceType::Wikidata,
        name: "Rigaku Smartlab Pole Figure raw format",
        extensions: &["asc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x54, 0x59, 0x50, 0x45, 0x09, 0x09, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
