use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851053: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_053,
        source_type: SourceType::Wikidata,
        name: "TRSI Sound Monitor Instrument",
        extensions: &["tsi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x53, 0x49, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
