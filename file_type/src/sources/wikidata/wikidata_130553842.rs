use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130553842: FileFormat = FileFormat {
    id: 130_553_842,
    puid: "wikidata/130553842",
    name: "QVT Operational Mapping language file format",
    extensions: &["qvto"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
