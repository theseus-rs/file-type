use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863518: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_518,
        source_type: SourceType::Wikidata,
        name: "Cocktail File Module",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x63, 0x6B, 0x74, 0x61, 0x69, 0x6C, 0x20, 0x46, 0x69, 0x6C,
                        0x65, 0x20, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x20, 0x20, 0x28, 0x63,
                        0x29, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
