use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_99972520: FileFormat = FileFormat {
    id: 99_972_520,
    puid: "wikidata/99972520",
    name: "OmniPage Pro Document 10",
    extensions: &["opd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
