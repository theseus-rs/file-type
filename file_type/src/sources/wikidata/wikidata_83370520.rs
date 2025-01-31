use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83370520: FileFormat = FileFormat {
    id: 83_370_520,
    puid: "wikidata/83370520",
    name: "Midi Sample dump Format",
    extensions: &["sds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
