use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_183: FileFormat = FileFormat {
    id: 256,
    puid: "x-fmt/183",
    name: "RealAudio Metafile",
    extensions: &["ram"],
    media_types: &["audio/vnd.rn-realaudio", "audio/x-pn-realaudio"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x74, 0x73, 0x70, 0x3A, 0x2F, 0x2F])],
            },
        }],
    }],
    related_formats: &[],
};
