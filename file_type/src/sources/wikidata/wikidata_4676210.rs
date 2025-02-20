use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4676210: FileType = FileType {
    file_format: &FileFormat {
        id: 4_676_210,
        source_type: SourceType::Wikidata,
        name: "Adaptive Multi-Rate audio codec",
        extensions: &["3ga", "amr"],
        media_types: &["audio/AMR"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x21, 0x41, 0x4D, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
