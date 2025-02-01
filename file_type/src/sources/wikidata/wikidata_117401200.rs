use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117401200: FileFormat = FileFormat {
    id: 117_401_200,
    puid: "wikidata/117401200",
    name: "Asymetrix Toolbook File 6-11.5",
    extensions: &["sbk", "tbk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
