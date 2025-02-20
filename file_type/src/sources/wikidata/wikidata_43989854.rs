use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_43989854: FileType = FileType {
    file_format: &FileFormat {
        id: 43_989_854,
        source_type: SourceType::Wikidata,
        name: "ATCO-CIF",
        extensions: &["cif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x54, 0x43, 0x4F, 0x2D, 0x43, 0x49, 0x46,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
