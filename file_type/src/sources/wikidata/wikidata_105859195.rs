use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859195: FileFormat = FileFormat {
    id: 105_859_195,
    source_type: SourceType::Wikidata,
    name: "CompuServe RLE bitmap (med-res)",
    extensions: &["rle"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1B, 0x47, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
