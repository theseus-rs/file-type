use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857912: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_912,
        source_type: SourceType::Wikidata,
        name: "Amigaguide Index",
        extensions: &["index"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x79, 0x70, 0x65, 0x72, 0x49, 0x6E, 0x64, 0x65, 0x78, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
