use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850797: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_797,
        source_type: SourceType::Wikidata,
        name: "Kid Pix Presentation",
        extensions: &["kpp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x61, 0x67, 0x65, 0x73, 0x5D, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
