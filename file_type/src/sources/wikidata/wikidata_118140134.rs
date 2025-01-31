use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118140134: FileFormat = FileFormat {
    id: 118_140_134,
    puid: "wikidata/118140134",
    name: "Serenade Project File",
    extensions: &["ssp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
