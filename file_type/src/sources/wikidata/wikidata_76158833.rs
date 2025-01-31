use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76158833: FileFormat = FileFormat {
    id: 76_158_833,
    puid: "wikidata/76158833",
    name: "Very Ordinary Raster file format bitmap",
    extensions: &["vort"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x4F, 0x52, 0x54, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
