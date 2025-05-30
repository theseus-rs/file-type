use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859195: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_195,
        source_type: SourceType::Wikidata,
        name: "CompuServe RLE bitmap (med-res)",
        extensions: &["rle"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1B, 0x47, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
