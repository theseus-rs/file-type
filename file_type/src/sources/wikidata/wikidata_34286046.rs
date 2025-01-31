use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34286046: FileFormat = FileFormat {
    id: 34_286_046,
    puid: "wikidata/34286046",
    name: "Pixie script",
    extensions: &["pxi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
