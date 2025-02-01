use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205858: FileFormat = FileFormat {
    id: 28_205_858,
    puid: "wikidata/28205858",
    name: "CompuServe RLE",
    extensions: &["rle"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1B, 0x47, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
