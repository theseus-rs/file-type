use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205340: FileFormat = FileFormat {
    id: 28_205_340,
    puid: "wikidata/28205340",
    name: "Hasselblad 3F RAW",
    extensions: &["3fr"],
    media_types: &["image/x-raw-hasselblad"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x49, 0x2A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
