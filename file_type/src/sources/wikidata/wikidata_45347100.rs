use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_45347100: FileFormat = FileFormat {
    id: 45_347_100,
    puid: "wikidata/45347100",
    name: "Lotus 1-2-3 Worksheet file format, version 1",
    extensions: &["wks"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x02, 0x00, 0x04, 0x04, 0x06, 0x00, 0x08, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
