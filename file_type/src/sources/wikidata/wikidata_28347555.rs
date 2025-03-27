use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28347555: FileType = FileType {
    file_format: &FileFormat {
        id: 28_347_555,
        source_type: SourceType::Wikidata,
        name: "IBM Audio Visual Connection (AVC) Still Video Image",
        extensions: &["!im", "_im", "id"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2B, 0x41, 0x2B, 0x56, 0x2B, 0x43, 0x2B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
