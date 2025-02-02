use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130142778: FileFormat = FileFormat {
    id: 130_142_778,
    source_type: SourceType::Wikidata,
    name: "OpenLDAP configuration file",
    extensions: &["ldaprc"],
    media_types: &["text/x-ldapconf"],
    internal_signatures: &[],
    related_formats: &[],
};
