use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113674482: FileFormat = FileFormat {
    id: 113_674_482,
    puid: "wikidata/113674482",
    name: "3D Landscape 2.0 File",
    extensions: &["3sl", "lnd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
