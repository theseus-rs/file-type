use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855893: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_893,
        source_type: SourceType::Wikidata,
        name: "FL Studio DrumKit (generic)",
        extensions: &["dmkit"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4D, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
