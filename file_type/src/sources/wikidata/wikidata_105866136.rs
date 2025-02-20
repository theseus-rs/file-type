use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866136: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_136,
        source_type: SourceType::Wikidata,
        name: "QNX Photon Font (bitmap)",
        extensions: &["phf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x51, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
