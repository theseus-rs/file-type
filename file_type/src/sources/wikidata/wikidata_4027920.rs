use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_4027920: FileType = FileType {
    file_format: &FileFormat {
        id: 4_027_920,
        source_type: SourceType::Wikidata,
        name: "XAP",
        extensions: &["xap"],
        media_types: &["application/x-silverlight-app"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
