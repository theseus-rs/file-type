use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28106135: FileType = FileType {
    file_format: &FileFormat {
        id: 28_106_135,
        source_type: SourceType::Wikidata,
        name: "Pi",
        extensions: &["pi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x69])],
                },
            }],
        }],
        related_formats: &[],
    },
};
