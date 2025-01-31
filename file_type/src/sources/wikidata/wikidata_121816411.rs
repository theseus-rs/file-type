use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121816411: FileFormat = FileFormat {
    id: 121_816_411,
    puid: "wikidata/121816411",
    name: "vCard 4",
    extensions: &["vcard", "vcf"],
    media_types: &["text/vcard", "text/vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
