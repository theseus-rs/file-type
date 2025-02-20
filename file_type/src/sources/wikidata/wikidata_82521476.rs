use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_82521476: FileType = FileType {
    file_format: &FileFormat {
        id: 82_521_476,
        source_type: SourceType::Wikidata,
        name: "Ensoniq PARIS",
        extensions: &["paf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x61, 0x70, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
