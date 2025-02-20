use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855803: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_803,
        source_type: SourceType::Wikidata,
        name: "Dream X2 Preset Format",
        extensions: &["dxp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB9, 0xF6, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
