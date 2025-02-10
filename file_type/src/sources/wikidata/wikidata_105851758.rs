use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851758: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_758,
        source_type: SourceType::Wikidata,
        name: "Children of the Nile campaign",
        extensions: &["ssa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x72, 0x61, 0x73, 0x73, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
