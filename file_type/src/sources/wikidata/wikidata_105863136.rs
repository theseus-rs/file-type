use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863136: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_136,
        source_type: SourceType::Wikidata,
        name: "Mizar article",
        extensions: &["miz"],
        media_types: &["text/mizar"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x65, 0x6E, 0x76, 0x69, 0x72, 0x6F, 0x6E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
