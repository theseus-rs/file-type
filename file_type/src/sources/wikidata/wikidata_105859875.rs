use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859875: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_875,
        source_type: SourceType::Wikidata,
        name: "VCG graph",
        extensions: &["vcg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x67, 0x72, 0x61, 0x70, 0x68, 0x3A, 0x20, 0x7B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
