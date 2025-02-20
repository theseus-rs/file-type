use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863260: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_260,
        source_type: SourceType::Wikidata,
        name: "Miva Script",
        extensions: &["mv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
