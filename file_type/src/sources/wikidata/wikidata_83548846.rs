use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83548846: FileFormat = FileFormat {
    id: 83_548_846,
    puid: "wikidata/83548846",
    name: "Nearly Raw Raster Data, version 3",
    extensions: &["nrrd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
