use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66210170: FileFormat = FileFormat {
    id: 66_210_170,
    puid: "wikidata/66210170",
    name: "FrameMaker Book file format",
    extensions: &["book"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
