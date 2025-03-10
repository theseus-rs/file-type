use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205687: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_687,
        source_type: SourceType::Wikidata,
        name: "PWG Raster",
        extensions: &[],
        media_types: &["image/pwg-raster"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x61, 0x53, 0x32, 0x50, 0x77, 0x67, 0x52, 0x61, 0x73, 0x74, 0x65,
                        0x72, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
