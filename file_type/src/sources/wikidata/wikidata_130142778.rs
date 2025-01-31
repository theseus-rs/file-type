use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130142778: FileFormat = FileFormat {
    id: 130_142_778,
    puid: "wikidata/130142778",
    name: "OpenLDAP configuration file",
    extensions: &["ldaprc"],
    media_types: &["text/x-ldapconf"],
    internal_signatures: &[],
    related_formats: &[],
};
