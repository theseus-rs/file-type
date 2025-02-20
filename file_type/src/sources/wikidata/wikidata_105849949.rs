use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849949: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_949,
        source_type: SourceType::Wikidata,
        name: "Cramfs ROM filesystem package (little endian)",
        extensions: &["cmg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x3D, 0xCD, 0x28])],
                },
            }],
        }],
        related_formats: &[],
    },
};
