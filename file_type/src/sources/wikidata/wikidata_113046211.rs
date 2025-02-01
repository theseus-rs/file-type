use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113046211: FileFormat = FileFormat {
    id: 113_046_211,
    puid: "wikidata/113046211",
    name: "Live Code File Format",
    extensions: &["mlx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
