use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205690: FileFormat = FileFormat {
    id: 28_205_690,
    puid: "wikidata/28205690",
    name: "AMOS Sprite Bank",
    extensions: &["abk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x6D, 0x53, 0x70, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
