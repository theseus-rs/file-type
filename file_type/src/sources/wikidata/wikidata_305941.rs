use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_305941: FileFormat = FileFormat {
    id: 305_941,
    puid: "wikidata/305941",
    name: "vCard",
    extensions: &["vcard", "vcf"],
    media_types: &["text/vcard", "text/vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
