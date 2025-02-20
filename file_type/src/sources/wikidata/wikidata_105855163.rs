use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855163: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_163,
        source_type: SourceType::Wikidata,
        name: "Amiga bitmap Font (var.2)",
        extensions: &["font"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0F, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
