use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121816030: FileFormat = FileFormat {
    id: 121_816_030,
    puid: "wikidata/121816030",
    name: "vCard 2.1",
    extensions: &["vcard", "vcf"],
    media_types: &["text/vcard", "text/vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
