use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111530722: FileFormat = FileFormat {
    id: 111_530_722,
    source_type: SourceType::Wikidata,
    name: "SGML/XML Entity File",
    extensions: &["ent"],
    media_types: &["application/xml-external-parsed-entity"],
    signatures: &[],
    related_formats: &[],
};
