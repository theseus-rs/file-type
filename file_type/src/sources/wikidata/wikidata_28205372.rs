use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205372: FileFormat = FileFormat {
    id: 28_205_372,
    puid: "wikidata/28205372",
    name: "Kodak TIFF",
    extensions: &["tif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
