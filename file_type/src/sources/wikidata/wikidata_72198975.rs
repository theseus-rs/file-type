use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_72198975: FileType = FileType {
    file_format: &FileFormat {
        id: 72_198_975,
        source_type: SourceType::Wikidata,
        name: "ABBYY Lingvo dictionary file format",
        extensions: &["lsd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x69, 0x6E, 0x67, 0x56, 0x6F, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
