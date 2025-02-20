use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856472: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_472,
        source_type: SourceType::Wikidata,
        name: "GFA Raytrace project data (low-res)",
        extensions: &["wfl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x77, 0x66, 0x6C, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
