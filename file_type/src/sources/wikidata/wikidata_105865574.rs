use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865574: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_574,
        source_type: SourceType::Wikidata,
        name: "Topocad coordinates",
        extensions: &["pxy"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x59, 0x5A, 0x2D, 0x43, 0x4F, 0x4F, 0x52, 0x44, 0x2D, 0x46, 0x49,
                        0x4C, 0x45, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
