use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857844: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_844,
        source_type: SourceType::Wikidata,
        name: "Q-emuLator Microdrive image",
        extensions: &["mdv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x64, 0x76, 0x2A, 0x44, 0x75, 0x6D, 0x70,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
