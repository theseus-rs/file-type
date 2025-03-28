use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855444: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_444,
        source_type: SourceType::Wikidata,
        name: "FidoCAD drawing",
        extensions: &["fcd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x46, 0x49, 0x44, 0x4F, 0x43, 0x41, 0x44,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
