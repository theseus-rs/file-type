use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855197: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_197,
        source_type: SourceType::Wikidata,
        name: "YS FLIGHT scenery field",
        extensions: &["fld"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x49, 0x45, 0x4C, 0x44, 0x0D, 0x0A, 0x47, 0x4E, 0x44, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
