use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850731: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_731,
        source_type: SourceType::Wikidata,
        name: "Kcov coverage data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6B, 0x63, 0x6F, 0x76])],
                },
            }],
        }],
        related_formats: &[],
    },
};
