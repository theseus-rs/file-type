use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76158833: FileType = FileType {
    file_format: &FileFormat {
        id: 76_158_833,
        source_type: SourceType::Wikidata,
        name: "Very Ordinary Raster file format bitmap",
        extensions: &["vort"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x4F, 0x52, 0x54, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
