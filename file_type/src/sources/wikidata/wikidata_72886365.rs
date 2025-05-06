use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72886365: FileType = FileType {
    file_format: &FileFormat {
        id: 72_886_365,
        source_type: SourceType::Wikidata,
        name: "PIK",
        extensions: &["pik"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD7, 0x4C, 0x4D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
