use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857139: FileFormat = FileFormat {
    id: 105_857_139,
    puid: "wikidata/105857139",
    name: "Houdini Apprentice Project",
    extensions: &["hipnc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x6F, 0x75, 0x4E, 0x43, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
