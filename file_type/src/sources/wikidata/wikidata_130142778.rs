use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130142778: FileFormat = FileFormat {
    id: 130_142_778,
    source_type: SourceType::Wikidata,
    name: "OpenLDAP configuration file",
    extensions: &["ldaprc"],
    media_types: &["text/x-ldapconf"],
    signatures: &[],
    related_formats: &[],
};
