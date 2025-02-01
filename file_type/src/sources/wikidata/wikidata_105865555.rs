use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865555: FileFormat = FileFormat {
    id: 105_865_555,
    puid: "wikidata/105865555",
    name: "Microsoft Purble Shop Saved game",
    extensions: &["purbleshopsave-ms"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x47, 0x4D, 0x48, 0x01, 0x00, 0x00, 0x00, 0x28, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
