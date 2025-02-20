use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854904: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_904,
        source_type: SourceType::Wikidata,
        name: "Maxis XA Audio (generic)",
        extensions: &["xa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x41, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
