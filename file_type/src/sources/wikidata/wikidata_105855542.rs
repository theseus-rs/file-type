use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855542: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_542,
        source_type: SourceType::Wikidata,
        name: "DB/TextWorks Database Terms and Words",
        extensions: &["occ"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x43, 0x43, 0x20, 0x30, 0x30, 0x35, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
