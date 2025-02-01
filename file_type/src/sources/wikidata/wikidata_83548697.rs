use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83548697: FileFormat = FileFormat {
    id: 83_548_697,
    puid: "wikidata/83548697",
    name: "Nearly Raw Raster Data, version 1",
    extensions: &["nrrd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
