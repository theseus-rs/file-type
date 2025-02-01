use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853967: FileFormat = FileFormat {
    id: 105_853_967,
    puid: "wikidata/105853967",
    name: "Xpack compressed archive (Win)",
    extensions: &["xpa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x78, 0x70, 0x61, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
