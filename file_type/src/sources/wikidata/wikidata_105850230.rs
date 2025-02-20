use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850230: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_230,
        source_type: SourceType::Wikidata,
        name: "EISA add-on card Configuration (with rem)",
        extensions: &["cfg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
