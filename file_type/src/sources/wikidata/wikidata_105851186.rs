use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851186: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_186,
        source_type: SourceType::Wikidata,
        name: "Nintendo 3DS VC ROM",
        extensions: &["nes"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x4E, 0x45, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
