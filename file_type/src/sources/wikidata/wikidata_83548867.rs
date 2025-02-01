use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83548867: FileFormat = FileFormat {
    id: 83_548_867,
    puid: "wikidata/83548867",
    name: "Nearly Raw Raster Data, version 4",
    extensions: &["nrrd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
