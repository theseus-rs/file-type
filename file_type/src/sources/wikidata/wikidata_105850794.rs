use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850794: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_794,
        source_type: SourceType::Wikidata,
        name: "Katorzer music",
        extensions: &["kat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x61, 0x54, 0x6F, 0x72, 0x5A, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
