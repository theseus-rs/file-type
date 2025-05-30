use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865400: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_400,
        source_type: SourceType::Wikidata,
        name: "Portable Font Resource",
        extensions: &["pfr"],
        media_types: &["application/font-tdpfr"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x46, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
