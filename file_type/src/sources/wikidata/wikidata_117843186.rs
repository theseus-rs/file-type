use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117843186: FileFormat = FileFormat {
    id: 117_843_186,
    puid: "wikidata/117843186",
    name: "Calculus EZ-Fax file",
    extensions: &["ezf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
