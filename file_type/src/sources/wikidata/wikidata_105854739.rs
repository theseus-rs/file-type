use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854739: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_739,
        source_type: SourceType::Wikidata,
        name: "Asura engine Resources package (generic)",
        extensions: &["asr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x73, 0x75, 0x72, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
