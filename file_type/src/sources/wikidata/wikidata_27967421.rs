use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967421: FileFormat = FileFormat {
    id: 27_967_421,
    puid: "wikidata/27967421",
    name: "CapXML",
    extensions: &["capx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
