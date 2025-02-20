use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850963: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_963,
        source_type: SourceType::Wikidata,
        name: "SuperJAM! Toot",
        extensions: &["toot"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x4F, 0x4F, 0x54, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
