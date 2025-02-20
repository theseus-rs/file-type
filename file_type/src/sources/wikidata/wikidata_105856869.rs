use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856869: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_869,
        source_type: SourceType::Wikidata,
        name: "Gravis UltraSound PnP InterWave patch",
        extensions: &["fff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x46, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
