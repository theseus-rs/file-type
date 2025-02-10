use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857229: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_229,
        source_type: SourceType::Wikidata,
        name: "FL Studio Hardcore Program",
        extensions: &["hdprg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0x52, 0x43, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
