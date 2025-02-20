use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967148: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_148,
        source_type: SourceType::Wikidata,
        name: "Extreme's Tracker module",
        extensions: &["ams"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x78, 0x74, 0x72, 0x65, 0x6D, 0x65, 0x30, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
