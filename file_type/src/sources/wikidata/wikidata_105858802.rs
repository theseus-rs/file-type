use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858802: FileFormat = FileFormat {
    id: 105_858_802,
    puid: "wikidata/105858802",
    name: "Open Cascade Technology 3D model",
    extensions: &["brep"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x42, 0x52, 0x65, 0x70, 0x5F, 0x44, 0x72, 0x61, 0x77, 0x61, 0x62, 0x6C,
                    0x65, 0x53, 0x68, 0x61, 0x70, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
