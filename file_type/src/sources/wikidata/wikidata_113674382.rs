use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113674382: FileFormat = FileFormat {
    id: 113_674_382,
    puid: "wikidata/113674382",
    name: "File Converter Document",
    extensions: &["fcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
