use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111530722: FileFormat = FileFormat {
    id: 111_530_722,
    puid: "wikidata/111530722",
    name: "SGML/XML Entity File",
    extensions: &["ent"],
    media_types: &["application/xml-external-parsed-entity"],
    internal_signatures: &[],
    related_formats: &[],
};
