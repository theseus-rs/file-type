use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857879: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_879,
        source_type: SourceType::Wikidata,
        name: "Picasa album info",
        extensions: &["ini"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x69, 0x63, 0x61, 0x73, 0x61, 0x5D, 0x0D, 0x0A, 0x6E, 0x61,
                        0x6D, 0x65, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
