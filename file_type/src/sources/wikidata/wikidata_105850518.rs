use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850518: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_518,
        source_type: SourceType::Wikidata,
        name: "CadStd drawing",
        extensions: &["cad"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x41, 0x44, 0x53, 0x54, 0x44, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
