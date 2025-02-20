use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862165: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_165,
        source_type: SourceType::Wikidata,
        name: "Mariner Write Document",
        extensions: &["mwd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x46, 0x46, 0x46, 0x30, 0x30, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
