use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855813: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_813,
        source_type: SourceType::Wikidata,
        name: "Zinc Data",
        extensions: &["dat", "z_t", "znc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x69, 0x6E, 0x63, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69,
                        0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
