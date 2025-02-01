use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113324648: FileFormat = FileFormat {
    id: 113_324_648,
    puid: "wikidata/113324648",
    name: "Pixlr Layered Image",
    extensions: &["pxd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
