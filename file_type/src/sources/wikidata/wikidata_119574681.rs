use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119574681: FileFormat = FileFormat {
    id: 119_574_681,
    puid: "wikidata/119574681",
    name: "Kid Pix File",
    extensions: &["kpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
