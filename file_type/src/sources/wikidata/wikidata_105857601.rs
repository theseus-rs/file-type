use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857601: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_601,
        source_type: SourceType::Wikidata,
        name: "ADFS disk image",
        extensions: &["adf", "adfs", "adl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x75, 0x67, 0x6F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
