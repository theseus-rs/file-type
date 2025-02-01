use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205944: FileFormat = FileFormat {
    id: 28_205_944,
    puid: "wikidata/28205944",
    name: "Doré Raster",
    extensions: &["dore", "img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
