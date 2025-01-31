use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_97033396: FileFormat = FileFormat {
    id: 97_033_396,
    puid: "wikidata/97033396",
    name: "Magick Vector Graphics",
    extensions: &["mvg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
