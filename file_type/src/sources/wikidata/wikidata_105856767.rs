use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856767: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_767,
        source_type: SourceType::Wikidata,
        name: "Dragon UnPACKer HTML Template",
        extensions: &["uht"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x55, 0x48, 0x54, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
