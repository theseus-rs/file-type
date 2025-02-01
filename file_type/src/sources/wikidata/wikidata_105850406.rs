use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850406: FileFormat = FileFormat {
    id: 105_850_406,
    puid: "wikidata/105850406",
    name: "ep32 Configuration",
    extensions: &["cfg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x50, 0x5F, 0x50, 0x41, 0x47, 0x45, 0x3D, 0x23,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
