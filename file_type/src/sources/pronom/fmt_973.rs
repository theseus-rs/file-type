use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_973: FileFormat = FileFormat {
    id: 1_778,
    puid: "fmt/973",
    name: "DTS Coherent Acoustics (DCA) Audio",
    extensions: &["dts"],
    media_types: &["audio/vnd.dts"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7F, 0xFE, 0x80, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
