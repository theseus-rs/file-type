use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51954521: FileFormat = FileFormat {
    id: 51_954_521,
    puid: "wikidata/51954521",
    name: "Microsoft FoxPro Database, version 2.6",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
