use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121816127: FileFormat = FileFormat {
    id: 121_816_127,
    puid: "wikidata/121816127",
    name: "vCard 3",
    extensions: &["vcard", "vcf"],
    media_types: &["text/vcard", "text/vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
