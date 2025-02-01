use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_86450854: FileFormat = FileFormat {
    id: 86_450_854,
    puid: "wikidata/86450854",
    name: "ASICS",
    extensions: &["asics"],
    media_types: &["application/vnd.etsi.asic-s+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
