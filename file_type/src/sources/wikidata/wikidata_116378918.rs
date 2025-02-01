use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116378918: FileFormat = FileFormat {
    id: 116_378_918,
    puid: "wikidata/116378918",
    name: "Approach Database File",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
