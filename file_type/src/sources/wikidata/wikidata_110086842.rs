use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110086842: FileFormat = FileFormat {
    id: 110_086_842,
    puid: "wikidata/110086842",
    name: "Agisoft Point Cloud",
    extensions: &["oc3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
