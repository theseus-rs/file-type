use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110254444: FileType = FileType {
    file_format: &FileFormat {
        id: 110_254_444,
        source_type: SourceType::Wikidata,
        name: "E-Seq Piano File",
        extensions: &["fil"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4F, 0x4D, 0x2D, 0x45, 0x53, 0x45, 0x51,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
