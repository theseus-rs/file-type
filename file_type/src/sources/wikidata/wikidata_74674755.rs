use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_74674755: FileFormat = FileFormat {
    id: 74_674_755,
    puid: "wikidata/74674755",
    name: "SPSS Table Look",
    extensions: &["tlo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
