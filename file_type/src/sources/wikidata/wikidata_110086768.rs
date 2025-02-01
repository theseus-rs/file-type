use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110086768: FileFormat = FileFormat {
    id: 110_086_768,
    puid: "wikidata/110086768",
    name: "Agisoft Project Archive",
    extensions: &["psz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
