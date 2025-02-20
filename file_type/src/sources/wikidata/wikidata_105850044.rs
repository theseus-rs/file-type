use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850044: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_044,
        source_type: SourceType::Wikidata,
        name: "Harvard Graphics Chart (v3.x)",
        extensions: &["ch3"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x47, 0x42, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
