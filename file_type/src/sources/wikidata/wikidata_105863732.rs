use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863732: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_732,
        source_type: SourceType::Wikidata,
        name: "m4 preprocessor / macro source",
        extensions: &["m4"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x6E, 0x6C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
