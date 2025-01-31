use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130393916: FileFormat = FileFormat {
    id: 130_393_916,
    puid: "wikidata/130393916",
    name: "Actual Drawing file",
    extensions: &["adf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
