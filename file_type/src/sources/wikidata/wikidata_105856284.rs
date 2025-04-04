use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856284: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_284,
        source_type: SourceType::Wikidata,
        name: "Diamond Caves 3 levels group",
        extensions: &["dc3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x43, 0x33, 0x4C, 0x56, 0x4C, 0x47, 0x52, 0x02, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
