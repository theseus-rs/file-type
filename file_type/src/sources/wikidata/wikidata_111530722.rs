use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111530722: FileFormat = FileFormat {
    id: 111_530_722,
    source_type: SourceType::Wikidata,
    name: "SGML/XML Entity File",
    extensions: &["ent"],
    media_types: &["application/xml-external-parsed-entity"],
    internal_signatures: &[],
    related_formats: &[],
};
