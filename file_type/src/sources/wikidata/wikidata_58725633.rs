use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58725633: FileFormat = FileFormat {
    id: 58_725_633,
    puid: "wikidata/58725633",
    name: "Adobe PostScript Font Metrics file",
    extensions: &["pfm"],
    media_types: &["application/x-font-pfm"],
    internal_signatures: &[],
    related_formats: &[],
};
