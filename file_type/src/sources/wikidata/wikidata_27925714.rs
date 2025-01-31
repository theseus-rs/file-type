use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27925714: FileFormat = FileFormat {
    id: 27_925_714,
    puid: "wikidata/27925714",
    name: "DTED Level 1 Gazetteer Hash file",
    extensions: &["hsh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
