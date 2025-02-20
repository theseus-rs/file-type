use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967397: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_397,
        source_type: SourceType::Wikidata,
        name: "Adlib Tracker II pattern",
        extensions: &["a2p"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5F, 0x61, 0x32, 0x70, 0x61, 0x74, 0x74, 0x65, 0x72, 0x6E, 0x5F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
