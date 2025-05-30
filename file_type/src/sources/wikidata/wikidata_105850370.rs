use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850370: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_370,
        source_type: SourceType::Wikidata,
        name: "Cult3D object",
        extensions: &["co"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x33, 0x44, 0x46, 0x30, 0x39, 0x34, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
