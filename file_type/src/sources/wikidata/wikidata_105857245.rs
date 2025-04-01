use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857245: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_245,
        source_type: SourceType::Wikidata,
        name: "HomeBrew Icon",
        extensions: &["hic"],
        media_types: &["application/octet-stream", "image/x-homebrew-icon"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x6F, 0x6D, 0x65, 0x42, 0x72, 0x65, 0x77, 0x20, 0x49, 0x63, 0x6F,
                        0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
