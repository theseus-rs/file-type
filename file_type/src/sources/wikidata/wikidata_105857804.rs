use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857804: FileFormat = FileFormat {
    id: 105_857_804,
    puid: "wikidata/105857804",
    name: "Virtual APF settings",
    extensions: &["ini"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4D, 0x45, 0x4D, 0x4F, 0x52, 0x59, 0x5D, 0x0D, 0x0A, 0x63, 0x61, 0x72,
                    0x74, 0x74, 0x79, 0x70, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
