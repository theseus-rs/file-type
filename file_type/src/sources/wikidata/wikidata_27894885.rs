use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27894885: FileType = FileType {
    file_format: &FileFormat {
        id: 27_894_885,
        source_type: SourceType::Wikidata,
        name: "Windows Media Video Redirector",
        extensions: &["wvx"],
        media_types: &["video/x-ms-wvx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x52, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6E, 0x63, 0x65, 0x5D, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
