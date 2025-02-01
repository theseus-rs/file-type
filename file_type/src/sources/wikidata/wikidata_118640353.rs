use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118640353: FileFormat = FileFormat {
    id: 118_640_353,
    puid: "wikidata/118640353",
    name: "Picture Definition file",
    extensions: &["lpd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
