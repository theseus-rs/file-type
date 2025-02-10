use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_1422885: FileType = FileType {
    file_format: &FileFormat {
        id: 1_422_885,
        source_type: SourceType::Wikidata,
        name: "MrSID",
        extensions: &["sid"],
        media_types: &["image/x-mrsid", "image/x-mrsid-image", "image/x.mrsid"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0x73, 0x69, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
