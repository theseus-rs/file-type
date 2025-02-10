use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862249: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_249,
        source_type: SourceType::Wikidata,
        name: "Musicline module",
        extensions: &["ml"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4C, 0x45, 0x44, 0x4D, 0x4F, 0x44, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
