use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_29876949: FileType = FileType {
    file_format: &FileFormat {
        id: 29_876_949,
        source_type: SourceType::Wikidata,
        name: "Clustal W",
        extensions: &["aln"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4C, 0x55, 0x53, 0x54, 0x41, 0x4C, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
