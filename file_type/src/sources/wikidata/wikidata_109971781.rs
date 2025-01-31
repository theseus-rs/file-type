use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109971781: FileFormat = FileFormat {
    id: 109_971_781,
    puid: "wikidata/109971781",
    name: "PDF Portfolio file format",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[],
    related_formats: &[],
};
