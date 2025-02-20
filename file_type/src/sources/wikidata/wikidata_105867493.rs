use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867493: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_493,
        source_type: SourceType::Wikidata,
        name: "NoiseTrekker v2.0 module",
        extensions: &["ntk"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x57, 0x4E, 0x4E, 0x53, 0x4E, 0x47, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
