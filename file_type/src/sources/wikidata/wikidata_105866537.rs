use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866537: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_537,
        source_type: SourceType::Wikidata,
        name: "Korg Trinity/Triton instruments bank (generic)",
        extensions: &["pcg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x4F, 0x52, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
