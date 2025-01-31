use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_74690858: FileFormat = FileFormat {
    id: 74_690_858,
    puid: "wikidata/74690858",
    name: "TomeRaider 3 eBook",
    extensions: &["tr3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x52, 0x33, 0x44, 0x54, 0x52, 0x33, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
