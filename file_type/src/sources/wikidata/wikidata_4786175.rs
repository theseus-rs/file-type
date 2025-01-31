use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4786175: FileFormat = FileFormat {
    id: 4_786_175,
    puid: "wikidata/4786175",
    name: "ArchiCAD library part",
    extensions: &["gsm"],
    media_types: &["model/vnd.gs-gdl"],
    internal_signatures: &[],
    related_formats: &[],
};
