use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857810: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_810,
        source_type: SourceType::Wikidata,
        name: "Toast disk image (old)",
        extensions: &["toast"],
        media_types: &["application/x-roxio-toast"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x52, 0x02, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
