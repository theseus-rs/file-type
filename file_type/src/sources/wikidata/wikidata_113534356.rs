use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113534356: FileFormat = FileFormat {
    id: 113_534_356,
    puid: "wikidata/113534356",
    name: "Pro Tools Session File",
    extensions: &["ptx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
