use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049379: FileFormat = FileFormat {
    id: 28_049_379,
    puid: "wikidata/28049379",
    name: "ComputerEyes Raw Data Format, medium resolution",
    extensions: &["ce2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x59, 0x45, 0x53, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
