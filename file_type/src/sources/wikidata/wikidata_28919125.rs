use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919125: FileFormat = FileFormat {
    id: 28_919_125,
    puid: "wikidata/28919125",
    name: "Final Cut Pro X project",
    extensions: &["fcpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
