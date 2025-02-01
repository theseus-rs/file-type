use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109944655: FileFormat = FileFormat {
    id: 109_944_655,
    puid: "wikidata/109944655",
    name: "Image Systems file format",
    extensions: &["igs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
