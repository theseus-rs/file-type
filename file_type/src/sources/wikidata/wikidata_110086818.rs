use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110086818: FileFormat = FileFormat {
    id: 110_086_818,
    puid: "wikidata/110086818",
    name: "Agisoft Project File",
    extensions: &["psx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
