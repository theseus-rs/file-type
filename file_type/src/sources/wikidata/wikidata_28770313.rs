use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770313: FileFormat = FileFormat {
    id: 28_770_313,
    source_type: SourceType::Wikidata,
    name: "LZX",
    extensions: &["lzx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x5A, 0x58, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
