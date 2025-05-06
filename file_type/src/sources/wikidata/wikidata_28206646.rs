use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206646: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_646,
        source_type: SourceType::Wikidata,
        name: "Multi Palette Picture",
        extensions: &["mpp"],
        media_types: &["application/octet-stream", "image/x-multi-palette-picture"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x50, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
