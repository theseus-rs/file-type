use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857885: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_885,
        source_type: SourceType::Wikidata,
        name: "Ozone GUI skin Info",
        extensions: &["ini"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x49, 0x6E, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                        0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
