use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855418: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_418,
        source_type: SourceType::Wikidata,
        name: "Fasoo Secure Container",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x21, 0x2D, 0x2D, 0x20, 0x46, 0x61, 0x73, 0x6F, 0x6F, 0x53, 0x65,
                        0x63, 0x75, 0x72, 0x65, 0x43, 0x6F, 0x6E, 0x74, 0x61, 0x69, 0x6E, 0x65,
                        0x72, 0x20, 0x2D, 0x20, 0x56, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
