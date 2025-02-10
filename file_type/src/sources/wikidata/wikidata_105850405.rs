use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850405: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_405,
        source_type: SourceType::Wikidata,
        name: "SuperJAM! Chords",
        extensions: &["chords"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x48, 0x52, 0x44, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
