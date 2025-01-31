use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967194: FileFormat = FileFormat {
    id: 27_967_194,
    puid: "wikidata/27967194",
    name: "Imago Orpheus module",
    extensions: &["imf"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4D, 0x31, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
