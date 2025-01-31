use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205725: FileFormat = FileFormat {
    id: 28_205_725,
    puid: "wikidata/28205725",
    name: "Async Professional Fax",
    extensions: &["apf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
