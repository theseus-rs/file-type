use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857544: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_544,
        source_type: SourceType::Wikidata,
        name: "MagicDesk Icon",
        extensions: &["icn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1F, 0x1F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
