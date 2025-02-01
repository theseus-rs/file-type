use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864305: FileFormat = FileFormat {
    id: 105_864_305,
    puid: "wikidata/105864305",
    name: "Caligari TrueSpace Polyline (v2.x)",
    extensions: &["poly"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x41, 0x4C, 0x42, 0x50, 0x4F, 0x4C, 0x59, 0x56, 0x45, 0x52, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
