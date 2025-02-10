use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_2997216: FileType = FileType {
    file_format: &FileFormat {
        id: 2_997_216,
        source_type: SourceType::Wikidata,
        name: "Core Audio Format",
        extensions: &["caf"],
        media_types: &["audio/x-caf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x63, 0x61, 0x66, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
