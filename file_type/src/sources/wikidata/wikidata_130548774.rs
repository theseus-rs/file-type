use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130548774: FileFormat = FileFormat {
    id: 130_548_774,
    puid: "wikidata/130548774",
    name: "Qlik file format",
    extensions: &["qvs", "qvw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
