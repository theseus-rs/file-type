use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853432: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_432,
        source_type: SourceType::Wikidata,
        name: "ZEsarUX snapshot",
        extensions: &["zx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x58, 0x00, 0xC0, 0x00, 0x40])],
                },
            }],
        }],
        related_formats: &[],
    },
};
