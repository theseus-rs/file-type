use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117866986: FileFormat = FileFormat {
    id: 117_866_986,
    puid: "wikidata/117866986",
    name: "American Data Tech SMARTFAX file",
    extensions: &["smf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
