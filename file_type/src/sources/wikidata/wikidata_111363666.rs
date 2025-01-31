use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111363666: FileFormat = FileFormat {
    id: 111_363_666,
    puid: "wikidata/111363666",
    name: "Wusikstation file-pack",
    extensions: &["wusikpack"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
